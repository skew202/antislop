//! Filename convention checking.
//!
//! This module detects AI-generated naming inconsistencies by learning from existing code:
//! - Learns conventions from existing files (requires 5+ files to establish pattern)
//! - Flags outliers that deviate from the established project style
//! - Detects potential duplicate files (common AI pattern)
//! - Respects existing tooling configuration (prettier, rustfmt, etc.)
//!
//! Philosophy: Don't enforce opinions. Learn what the project does and flag deviations.

use crate::config::{Pattern, PatternCategory, Severity};
use crate::detector::Finding;
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Configuration for filename checking behavior.
#[derive(Debug, Clone, Default)]
pub struct FilenameCheckConfig {
    /// If true, check for duplicate-like filenames (file_real.rs when file.rs exists)
    pub check_duplicates: bool,
    /// Minimum files required to establish a naming convention
    pub min_files_for_convention: usize,
    /// Threshold for convention deviation (0.0-1.0, where 1.0 = all files must match)
    /// Default 0.7 means 70% of files must follow a convention before deviations are flagged
    pub convention_threshold: f64,
}

/// Extract suffix patterns from naming patterns (for duplicate detection).
fn extract_duplicate_suffixes(patterns: &[Pattern]) -> Vec<String> {
    patterns
        .iter()
        .filter(|p| p.category == PatternCategory::NamingConvention)
        .filter_map(|p| {
            let regex_str = p.regex.to_string();
            // Suffix patterns look like: (?i)_real\.(rs|py)$
            // Match patterns that have _ followed by word chars before \.
            let suffix_re = regex::Regex::new(r"(?i)_[a-z_]+\\\.").unwrap();
            if !suffix_re.is_match(&regex_str) {
                return None;
            }
            // Extract the _word part
            if let Some(start) = regex_str.find('_') {
                if let Some(end) = regex_str[start..].find('\\') {
                    let suffix = &regex_str[start..start + end];
                    let suffix = suffix.trim_start_matches("(?i)").to_string();
                    return Some(suffix);
                }
            }
            None
        })
        .collect()
}

/// Extract prefix patterns from naming patterns (for duplicate detection).
fn extract_duplicate_prefixes(patterns: &[Pattern]) -> Vec<String> {
    patterns
        .iter()
        .filter(|p| p.category == PatternCategory::NamingConvention)
        .filter_map(|p| {
            let regex_str = p.regex.to_string();
            // Prefix patterns look like: (?i)^old_.*\.(rs|py)$
            // Match patterns that have ^word_ at the start
            let prefix_re = regex::Regex::new(r"\^\[?\^?\)?(?:\\i)?\)?([a-z_]+)_").unwrap();
            if let Some(caps) = prefix_re.captures(&regex_str) {
                if let Some(prefix) = caps.get(1) {
                    return Some(format!("_{}", prefix.as_str()));
                }
            }
            // Fallback: look for ^word_ pattern
            if regex_str.contains("^") && regex_str.contains("_.*\\.") {
                let after_start = regex_str.split('^').nth(1)?;
                let prefix_end = after_start.find('_')?;
                let prefix = &after_start[..prefix_end];
                // Remove (?i) and other regex constructs
                let prefix = prefix
                    .replace("(?i)", "")
                    .replace("[^]", "")
                    .replace("^", "");
                if !prefix.is_empty() && prefix.chars().all(|c| c.is_ascii_alphabetic() || c == '_')
                {
                    return Some(format!("_{}", prefix));
                }
            }
            None
        })
        .collect()
}

/// Detected tooling config files that indicate style preferences.
const TOOLING_CONFIGS: &[&str] = &[
    ".prettierrc",
    ".prettierrc.json",
    ".prettierrc.yaml",
    ".prettierrc.yml",
    "prettier.config.js",
    ".eslintrc",
    ".eslintrc.json",
    ".eslintrc.yml",
    "pyproject.toml",
    "setup.cfg",
    ".flake8",
    "rustfmt.toml",
    ".rustfmt.toml",
    "pylintrc",
    ".gofmt",
];

/// Naming convention type detected in a filename.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NamingConvention {
    /// snake_case (e.g., my_file_name.rs)
    SnakeCase,
    /// camelCase (e.g., myFileName.rs)
    CamelCase,
    /// PascalCase (e.g., MyFileName.rs)
    PascalCase,
    /// kebab-case (e.g., my-file-name.rs)
    KebabCase,
    /// Unknown or mixed convention
    Unknown,
}

impl NamingConvention {
    /// Detect the naming convention from a filename stem (without extension).
    fn detect(stem: &str) -> Self {
        let stem = stem.trim();

        // Check for kebab-case (contains hyphens)
        if stem.contains('-') {
            return NamingConvention::KebabCase;
        }

        // Check for snake_case (contains underscores)
        if stem.contains('_') {
            // Verify it's actually snake_case (all lowercase segments)
            let segments: Vec<&str> = stem.split('_').collect();
            if segments
                .iter()
                .all(|s| s.chars().all(|c| c.is_lowercase() || c.is_ascii_digit()))
            {
                return NamingConvention::SnakeCase;
            }
        }

        // Check for camelCase (starts lowercase, contains uppercase)
        let chars: Vec<char> = stem.chars().collect();
        if chars.len() > 1 {
            let has_upper = chars.iter().any(|c| c.is_uppercase());
            let starts_lower = chars.first().map(|c| c.is_lowercase()).unwrap_or(false);

            if has_upper && starts_lower {
                return NamingConvention::CamelCase;
            }

            // Check for PascalCase (starts uppercase, contains uppercase)
            let starts_upper = chars.first().map(|c| c.is_uppercase()).unwrap_or(false);
            if has_upper && starts_upper {
                return NamingConvention::PascalCase;
            }
        }

        // Single word, all lowercase - compatible with snake_case
        if stem.chars().all(|c| c.is_lowercase() || c.is_ascii_digit()) {
            return NamingConvention::SnakeCase;
        }

        // Single word, all uppercase - compatible with PascalCase
        if stem.chars().all(|c| c.is_uppercase() || c.is_ascii_digit()) {
            return NamingConvention::PascalCase;
        }

        NamingConvention::Unknown
    }

    /// Returns the expected convention for a language based on common standards.
    /// This is a weak hint - actual project convention takes precedence.
    #[allow(dead_code)]
    fn expected_for_language(extension: &str) -> Option<Self> {
        match extension {
            // Python strongly prefers snake_case for modules
            "py" => Some(NamingConvention::SnakeCase),
            // Rust prefers snake_case for modules
            "rs" => Some(NamingConvention::SnakeCase),
            // Go strongly prefers snake_case
            "go" => Some(NamingConvention::SnakeCase),
            // JavaScript/TypeScript - no strong preference, depends on tooling
            "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs" => None,
            // Java strongly prefers PascalCase for classes (but files match class names)
            "java" => Some(NamingConvention::PascalCase),
            // C# strongly prefers PascalCase
            "cs" => Some(NamingConvention::PascalCase),
            _ => None,
        }
    }
}

/// A group of files for convention analysis.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FileGroup {
    /// File extension (with dot, e.g., ".rs")
    extension: String,
    /// Parent directory path
    directory: String,
}

impl FileGroup {
    /// Create a file group from a path.
    fn from_path(path: &Path) -> Option<Self> {
        let extension = path.extension()?.to_str()?;
        let stem = path.file_stem()?.to_str()?;

        // Skip test files, benchmarks, mocks for convention analysis
        // These often legitimately use different naming
        if stem.contains("test") || stem.contains("spec") || stem.contains("mock") {
            return None;
        }

        let directory = path.parent()?.to_str()?.to_string();

        Some(Self {
            extension: format!(".{}", extension),
            directory,
        })
    }
}

/// Filename convention analyzer.
pub struct FilenameChecker {
    /// Files grouped by extension and directory for convention analysis.
    grouped_files: HashMap<FileGroup, Vec<String>>,
    /// All file paths for duplicate checking.
    all_files: Vec<String>,
    /// Configuration for checking behavior.
    config: FilenameCheckConfig,
    /// Detected tooling configuration files.
    tooling_configs: HashSet<String>,
    /// Suffix patterns for duplicate detection (from naming.toml).
    duplicate_suffixes: Vec<String>,
    /// Prefix patterns for duplicate detection (from naming.toml).
    duplicate_prefixes: Vec<String>,
}

impl FilenameChecker {
    /// Create a new filename checker with default configuration.
    pub fn new() -> Self {
        Self::with_config_and_patterns(FilenameCheckConfig::default(), &[])
    }

    /// Create a new filename checker with custom configuration.
    pub fn with_config(config: FilenameCheckConfig) -> Self {
        Self::with_config_and_patterns(config, &[])
    }

    /// Create a new filename checker with config and patterns (for duplicate detection).
    pub fn with_config_and_patterns(config: FilenameCheckConfig, patterns: &[Pattern]) -> Self {
        let (duplicate_suffixes, duplicate_prefixes) = if config.check_duplicates {
            (
                extract_duplicate_suffixes(patterns),
                extract_duplicate_prefixes(patterns),
            )
        } else {
            (Vec::new(), Vec::new())
        };

        Self {
            grouped_files: HashMap::new(),
            all_files: Vec::new(),
            config,
            tooling_configs: HashSet::new(),
            duplicate_suffixes,
            duplicate_prefixes,
        }
    }

    /// Add a file for analysis.
    pub fn add_file(&mut self, path: &Path) {
        let path_str = path.to_string_lossy().to_string();

        // Track tooling configs
        if let Some(filename) = path.file_name() {
            if let Some(name) = filename.to_str() {
                if TOOLING_CONFIGS.contains(&name) {
                    self.tooling_configs.insert(name.to_string());
                }
            }
        }

        self.all_files.push(path_str.clone());

        // Group for convention analysis
        if let Some(group) = FileGroup::from_path(path) {
            self.grouped_files.entry(group).or_default().push(path_str);
        }
    }

    /// Check all files for naming convention violations.
    pub fn check(&self) -> Vec<Finding> {
        let mut findings = Vec::new();

        // Check for potential duplicates
        if self.config.check_duplicates {
            findings.extend(self.check_duplicate_patterns());
        }

        // Check for convention breaks within groups
        findings.extend(self.check_convention_breaks());

        findings
    }

    /// Check for potential duplicate files (AI pattern of creating "alternatives").
    fn check_duplicate_patterns(&self) -> Vec<Finding> {
        let mut findings = Vec::new();

        // Group files by directory
        let mut by_dir: HashMap<&str, Vec<&str>> = HashMap::new();
        for path in &self.all_files {
            let path_obj = Path::new(path);
            if let Some(dir) = path_obj.parent().and_then(|p| p.to_str()) {
                by_dir.entry(dir).or_default().push(path);
            }
        }

        // Check each directory for duplicate patterns
        for files in by_dir.values() {
            let stems: Vec<(&str, &str)> = files
                .iter()
                .filter_map(|path| {
                    let path_obj = Path::new(path);
                    let stem = path_obj.file_stem()?.to_str()?;
                    let ext = path_obj.extension()?.to_str()?;
                    Some((stem, ext))
                })
                .collect();

            for (stem, ext) in &stems {
                let stem_lower = stem.to_lowercase();

                // Check suffix patterns (e.g., file_real.rs when file.rs exists)
                for pattern in &self.duplicate_suffixes {
                    let pattern_lower = pattern.to_lowercase();
                    if stem_lower.ends_with(&pattern_lower) {
                        // Extract base name (without suffix)
                        let base = stem_lower
                            .strip_suffix(&pattern_lower)
                            .unwrap_or(&stem_lower);

                        // Check if base file exists with same extension
                        let base_exists = stems
                            .iter()
                            .any(|(s, e)| *s == base && e == ext && *s != *stem);

                        if base_exists {
                            // Find the actual file path that contains this stem
                            let file_path_str = files
                                .iter()
                                .find(|p| p.contains(stem))
                                .copied()
                                .unwrap_or("");

                            findings.push(Finding {
                                file: file_path_str.to_string(),
                                line: 1,
                                column: 1,
                                severity: Severity::High,
                                category: crate::config::PatternCategory::NamingConvention,
                                message: format!(
                                    "Potential duplicate: '{}{}' exists alongside '{}{}'",
                                    base, ext, stem, ext
                                ),
                                match_text: format!("{}.{}", stem, ext),
                                pattern_regex: "duplicate_file".to_string(),
                            });
                        }
                        break;
                    }
                }

                // Check prefix patterns (e.g., new_file.rs when file.rs exists)
                for pattern in &self.duplicate_prefixes {
                    let pattern_lower = pattern.to_lowercase();
                    if stem_lower.starts_with(&pattern_lower) {
                        // Extract base name (without prefix)
                        let base = stem_lower
                            .strip_prefix(&pattern_lower)
                            .unwrap_or(&stem_lower);

                        // Check if base file exists with same extension
                        let base_exists = stems
                            .iter()
                            .any(|(s, e)| *s == base && e == ext && *s != *stem);

                        if base_exists {
                            // Find the actual file path that contains this stem
                            let file_path_str = files
                                .iter()
                                .find(|p| p.contains(stem))
                                .copied()
                                .unwrap_or("");

                            findings.push(Finding {
                                file: file_path_str.to_string(),
                                line: 1,
                                column: 1,
                                severity: Severity::High,
                                category: crate::config::PatternCategory::NamingConvention,
                                message: format!(
                                    "Potential duplicate: '{}{}' exists alongside '{}{}'",
                                    stem, ext, base, ext
                                ),
                                match_text: format!("{}.{}", stem, ext),
                                pattern_regex: "duplicate_file".to_string(),
                            });
                        }
                        break;
                    }
                }
            }
        }

        findings
    }

    /// Check for naming convention breaks within file groups.
    fn check_convention_breaks(&self) -> Vec<Finding> {
        let mut findings = Vec::new();

        for files in self.grouped_files.values() {
            // Need minimum files to establish a convention
            if files.len() < self.config.min_files_for_convention {
                continue;
            }

            let conventions: Vec<(NamingConvention, String)> = files
                .iter()
                .filter_map(|path| {
                    let path_obj = Path::new(path);
                    let stem = path_obj.file_stem()?.to_str()?;
                    Some((NamingConvention::detect(stem), path.clone()))
                })
                .collect();

            if conventions.is_empty() {
                continue;
            }

            // Count conventions
            let convention_counts = self.count_conventions(&conventions);

            // Find the dominant convention based on threshold
            let total = conventions.len() as f64;
            let dominant = convention_counts
                .iter()
                .find(|(_, &count)| {
                    let ratio = count as f64 / total;
                    ratio >= self.config.convention_threshold
                })
                .map(|(conv, _)| *conv);

            let dominant_convention = match dominant {
                Some(conv) => conv,
                None => {
                    // No clear dominant convention - don't flag anything
                    continue;
                }
            };

            // Flag outliers
            for (convention, path) in &conventions {
                if *convention != dominant_convention && *convention != NamingConvention::Unknown {
                    let filename = Path::new(path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("");

                    findings.push(Finding {
                        file: path.to_string(),
                        line: 1,
                        column: 1,
                        severity: Severity::Medium,
                        category: crate::config::PatternCategory::NamingConvention,
                        message: format!(
                            "Naming inconsistency: '{}' uses {:?} but project uses {:?}",
                            filename, *convention, dominant_convention
                        )
                        .to_lowercase()
                        .replace("snakecase", "snake_case")
                        .replace("camelcase", "camelCase")
                        .replace("pascalcase", "PascalCase")
                        .replace("kebabcase", "kebab-case"),
                        match_text: path.clone(),
                        pattern_regex: "naming_convention".to_string(),
                    });
                }
            }
        }

        findings
    }

    /// Count occurrences of each naming convention.
    fn count_conventions(
        &self,
        conventions: &[(NamingConvention, String)],
    ) -> HashMap<NamingConvention, usize> {
        let mut counts = HashMap::new();
        for (convention, _) in conventions {
            *counts.entry(*convention).or_insert(0) += 1;
        }
        counts
    }
}

impl Default for FilenameChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naming_convention_detection() {
        assert_eq!(
            NamingConvention::detect("my_file"),
            NamingConvention::SnakeCase
        );
        assert_eq!(
            NamingConvention::detect("myFile"),
            NamingConvention::CamelCase
        );
        assert_eq!(
            NamingConvention::detect("MyFile"),
            NamingConvention::PascalCase
        );
        assert_eq!(
            NamingConvention::detect("my-file"),
            NamingConvention::KebabCase
        );
        assert_eq!(
            NamingConvention::detect("myfile"),
            NamingConvention::SnakeCase
        );
        assert_eq!(
            NamingConvention::detect("MYFILE"),
            NamingConvention::PascalCase
        );
    }

    #[test]
    fn test_expected_convention_for_language() {
        assert_eq!(
            NamingConvention::expected_for_language("py"),
            Some(NamingConvention::SnakeCase)
        );
        assert_eq!(
            NamingConvention::expected_for_language("rs"),
            Some(NamingConvention::SnakeCase)
        );
        assert_eq!(
            NamingConvention::expected_for_language("go"),
            Some(NamingConvention::SnakeCase)
        );
        assert_eq!(
            NamingConvention::expected_for_language("java"),
            Some(NamingConvention::PascalCase)
        );
        assert_eq!(
            NamingConvention::expected_for_language("cs"),
            Some(NamingConvention::PascalCase)
        );
        assert_eq!(NamingConvention::expected_for_language("js"), None);
        assert_eq!(NamingConvention::expected_for_language("ts"), None);
    }

    #[test]
    fn test_duplicate_detection() {
        let config = FilenameCheckConfig {
            check_duplicates: true,
            min_files_for_convention: 5,
            convention_threshold: 0.7,
        };

        // Create mock patterns for testing
        let patterns = vec![
            Pattern {
                regex: crate::config::RegexPattern::new("(?i)_real\\.(rs|py)".to_string())
                    .expect("valid regex"),
                severity: Severity::High,
                message: "test".to_string(),
                category: PatternCategory::NamingConvention,
                ast_query: None,
                languages: vec![],
            },
            Pattern {
                regex: crate::config::RegexPattern::new("(?i)_new\\.(rs|py)".to_string())
                    .expect("valid regex"),
                severity: Severity::High,
                message: "test".to_string(),
                category: PatternCategory::NamingConvention,
                ast_query: None,
                languages: vec![],
            },
        ];

        let mut checker = FilenameChecker::with_config_and_patterns(config, &patterns);

        // Add original files
        checker.add_file(Path::new("/src/utils.rs"));
        checker.add_file(Path::new("/src/parser.rs"));

        // Add duplicate-like file
        checker.add_file(Path::new("/src/utils_real.rs"));
        checker.add_file(Path::new("/src/parser_new.rs"));

        let findings = checker.check_duplicate_patterns();
        assert_eq!(findings.len(), 2);
    }

    #[test]
    fn test_convention_break_detection_with_threshold() {
        let config = FilenameCheckConfig {
            check_duplicates: false,
            min_files_for_convention: 5,
            convention_threshold: 0.6, // 60% threshold
        };
        let mut checker = FilenameChecker::with_config(config);

        // Establish snake_case convention (4 out of 5 = 80%)
        checker.add_file(Path::new("/src/file_one.rs"));
        checker.add_file(Path::new("/src/file_two.rs"));
        checker.add_file(Path::new("/src/file_three.rs"));
        checker.add_file(Path::new("/src/file_four.rs"));

        // Add convention breaker
        checker.add_file(Path::new("/src/fileFive.rs"));

        let findings = checker.check_convention_breaks();
        assert_eq!(findings.len(), 1);
        assert!(findings[0].file.contains("fileFive"));
    }

    #[test]
    fn test_no_convention_break_below_threshold() {
        let config = FilenameCheckConfig {
            check_duplicates: false,
            min_files_for_convention: 5,
            convention_threshold: 0.8, // 80% threshold
        };
        let mut checker = FilenameChecker::with_config(config);

        // Mixed: 3 snake_case, 2 camelCase (60% snake, 40% camel)
        // Neither reaches 80% threshold
        checker.add_file(Path::new("/src/file_one.rs"));
        checker.add_file(Path::new("/src/file_two.rs"));
        checker.add_file(Path::new("/src/file_three.rs"));
        checker.add_file(Path::new("/src/fileFour.rs"));
        checker.add_file(Path::new("/src/fileFive.rs"));

        let findings = checker.check_convention_breaks();
        // No findings because no clear dominant convention
        assert_eq!(findings.len(), 0);
    }

    #[test]
    fn test_min_files_for_convention() {
        let config = FilenameCheckConfig {
            check_duplicates: false,
            min_files_for_convention: 10, // High threshold
            convention_threshold: 0.6,
        };
        let mut checker = FilenameChecker::with_config(config);

        // Only 5 files - not enough
        checker.add_file(Path::new("/src/file_one.rs"));
        checker.add_file(Path::new("/src/file_two.rs"));
        checker.add_file(Path::new("/src/file_three.rs"));
        checker.add_file(Path::new("/src/file_four.rs"));
        checker.add_file(Path::new("/src/fileFive.rs"));

        let findings = checker.check_convention_breaks();
        assert_eq!(findings.len(), 0);
    }

    #[test]
    fn test_grouped_by_directory() {
        let mut checker = FilenameChecker::new();

        // Different directories should have independent conventions
        checker.add_file(Path::new("/src/snake_file.rs"));
        checker.add_file(Path::new("/src/another_snake.rs"));
        checker.add_file(Path::new("/src/third_snake.rs"));
        checker.add_file(Path::new("/tests/PascalFile.rs"));
        checker.add_file(Path::new("/tests/AnotherPascal.rs"));
        checker.add_file(Path::new("/tests/ThirdPascal.rs"));

        // No findings expected - each dir follows its own convention
        let findings = checker.check_convention_breaks();
        assert_eq!(findings.len(), 0);
    }

    #[test]
    fn test_test_files_excluded_from_convention_check() {
        let mut checker = FilenameChecker::new();

        // Test files should be excluded
        checker.add_file(Path::new("/src/utils.rs"));
        checker.add_file(Path::new("/src/parser.rs"));
        checker.add_file(Path::new("/src/test_utils.rs")); // Excluded
        checker.add_file(Path::new("/src/spec_helper.rs")); // Excluded
        checker.add_file(Path::new("/src/mock_data.rs")); // Excluded

        // Only 2 non-test files - not enough to establish convention
        let findings = checker.check_convention_breaks();
        assert_eq!(findings.len(), 0);
    }

    #[test]
    fn test_full_check_with_defaults() {
        // Test with duplicate detection enabled explicitly
        let config = FilenameCheckConfig {
            check_duplicates: true,
            min_files_for_convention: 5,
            convention_threshold: 0.7,
        };

        // Create mock patterns for testing
        let patterns = vec![Pattern {
            regex: crate::config::RegexPattern::new("(?i)_real\\.(rs|py)".to_string())
                .expect("valid regex"),
            severity: Severity::High,
            message: "test".to_string(),
            category: PatternCategory::NamingConvention,
            ast_query: None,
            languages: vec![],
        }];

        let mut checker = FilenameChecker::with_config_and_patterns(config, &patterns);

        checker.add_file(Path::new("/src/module.rs"));
        checker.add_file(Path::new("/src/helper.rs"));
        checker.add_file(Path::new("/src/parser.rs"));
        checker.add_file(Path::new("/src/reader.rs"));
        checker.add_file(Path::new("/src/writer.rs"));
        checker.add_file(Path::new("/src/module.rs")); // Original
        checker.add_file(Path::new("/src/module_real.rs")); // Potential duplicate

        let findings = checker.check();
        // Should find the duplicate
        assert!(findings.iter().any(|f| f.message.contains("duplicate")));
    }
}
