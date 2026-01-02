//! Configuration loading and management.
//!
//! Antislop uses layered configuration: built-in defaults → config file → CLI overrides.

use crate::{Error, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const DEFAULT_CONFIG_TOML: &str = include_str!("../config/default.toml");

/// Embedded pattern definitions.
///
/// Patterns are stored as individual files in config/patterns/ and embedded at compile time.
/// This allows for easy maintenance while keeping zero runtime dependency on external files.
const PATTERNS_DIR: PatternsDirectory = PatternsDirectory::new();

/// Directory of embedded pattern files.
struct PatternsDirectory {
    files: &'static [(&'static str, &'static str)],
}

impl PatternsDirectory {
    /// Create a new patterns directory with all embedded pattern files.
    const fn new() -> Self {
        Self {
            files: &[
                (
                    "placeholder",
                    include_str!("../config/patterns/placeholder.toml"),
                ),
                ("deferral", include_str!("../config/patterns/deferral.toml")),
                ("hedging", include_str!("../config/patterns/hedging.toml")),
                ("stub", include_str!("../config/patterns/stub.toml")),
                ("ast", include_str!("../config/patterns/ast.toml")),
            ],
        }
    }

    /// Load all patterns from embedded files.
    fn load_patterns(&self) -> Vec<Pattern> {
        let mut all_patterns = Vec::new();

        for (name, content) in self.files {
            match self.parse_file(content) {
                Ok(mut patterns) => {
                    all_patterns.append(&mut patterns);
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to parse embedded patterns '{}': {}",
                        name, e
                    );
                }
            }
        }

        all_patterns
    }

    /// Parse patterns from a single TOML file.
    fn parse_file(&self, content: &str) -> Result<Vec<Pattern>> {
        let parsed: PatternFile = toml::from_str(content)
            .map_err(|e| Error::ConfigInvalid(format!("Parse error in pattern file: {}", e)))?;
        Ok(parsed.patterns)
    }
}

/// Pattern file structure (TOML).
#[derive(Debug, Deserialize)]
struct PatternFile {
    patterns: Vec<Pattern>,
}

/// Regex pattern with validation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "String", into = "String")]
pub struct RegexPattern(String);

impl RegexPattern {
    pub fn new(s: String) -> std::result::Result<Self, regex::Error> {
        regex::Regex::new(&s)?;
        Ok(Self(s))
    }
}

impl TryFrom<String> for RegexPattern {
    type Error = regex::Error;
    fn try_from(s: String) -> std::result::Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl From<RegexPattern> for String {
    fn from(val: RegexPattern) -> Self {
        val.0
    }
}

impl std::ops::Deref for RegexPattern {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Severity level for a slop finding.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// Minor issue, worth addressing but not urgent.
    Low,
    /// Moderate issue, should be fixed.
    #[default]
    Medium,
    /// Significant issue, fix recommended.
    High,
    /// Critical issue requiring immediate attention.
    Critical,
}

impl Severity {
    /// Returns the numeric score for this severity level.
    pub fn score(&self) -> u32 {
        match self {
            Severity::Low => 1,
            Severity::Medium => 5,
            Severity::High => 15,
            Severity::Critical => 50,
        }
    }

    /// Returns the display name for this severity.
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Low => "LOW",
            Severity::Medium => "MEDIUM",
            Severity::High => "HIGH",
            Severity::Critical => "CRITICAL",
        }
    }
}

/// Category of slop pattern.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum PatternCategory {
    /// Placeholder comments: TODO, FIXME, HACK, etc.
    #[default]
    Placeholder,
    /// Deferral language: "for now", "temporary", etc.
    Deferral,
    /// Hedging language: "hopefully", "should work", etc.
    Hedging,
    /// Stub code: empty implementations.
    Stub,
}

/// A single slop detection pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    /// Regular expression to match (case-insensitive supported with (?i)).
    pub regex: RegexPattern,
    /// Severity level for matches.
    #[serde(default)]
    pub severity: Severity,
    /// Human-readable description.
    #[serde(default)]
    pub message: String,
    /// Category of slop this pattern detects.
    #[serde(default)]
    pub category: PatternCategory,
    /// Optional tree-sitter query for AST-level detection.
    /// If provided, this pattern uses AST queries instead of regex.
    #[serde(default)]
    pub ast_query: Option<String>,
    /// Languages this AST query applies to (e.g., ["Python", "JavaScript"]).
    /// Only used when ast_query is set.
    #[serde(default)]
    pub languages: Vec<String>,
}

/// Main configuration structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Detection patterns.
    #[serde(default)]
    pub patterns: Vec<Pattern>,
    /// Glob patterns for paths to exclude.
    #[serde(default)]
    pub exclude: Vec<String>,
    /// Additional glob patterns for exclusion.
    #[serde(default)]
    pub exclude_patterns: Vec<String>,
    /// File extensions to scan.
    #[serde(default = "default_extensions")]
    pub file_extensions: Vec<String>,
    /// Maximum file size to scan in KB.
    #[serde(default = "default_max_file_size")]
    pub max_file_size_kb: u64,
}

fn default_extensions() -> Vec<String> {
    vec![
        ".rs", ".py", ".js", ".ts", ".jsx", ".tsx", ".go", ".java", ".kt", ".c", ".cpp", ".h",
        ".hpp", ".cs", ".php", ".rb", ".swift", ".dart",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect()
}

fn default_max_file_size() -> u64 {
    1024
}

impl Default for Config {
    fn default() -> Self {
        let mut base: Config =
            toml::from_str(DEFAULT_CONFIG_TOML).expect("default config must be valid");
        // Load patterns from embedded pattern files
        base.patterns = PATTERNS_DIR.load_patterns();
        base
    }
}

impl Config {
    /// Load configuration from a file.
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path).map_err(|e| {
            Error::ConfigInvalid(format!(
                "Failed to open config file '{}': {}",
                path.display(),
                e
            ))
        })?;
        let config: Self = toml::from_str(&content)
            .map_err(|e| Error::ConfigInvalid(format!("Parse error: {}", e)))?;
        Ok(config)
    }

    /// Load from path if it exists, otherwise return default.
    pub fn load_or_default(path: Option<&Path>) -> Self {
        match path {
            Some(p) if p.exists() => Self::load(p).unwrap_or_else(|e| {
                eprintln!("Warning: Failed to load config from {}: {}", p.display(), e);
                Self::default()
            }),
            _ => Self::default(),
        }
    }

    /// Validate all regex patterns in the config.
    pub fn validate_patterns(&self) -> Result<()> {
        for pattern in &self.patterns {
            Regex::new(&pattern.regex).map_err(Error::Regex)?;
        }
        Ok(())
    }

    /// Get all patterns for a specific category.
    pub fn patterns_for_category(&self, category: &PatternCategory) -> Vec<&Pattern> {
        self.patterns
            .iter()
            .filter(|p| &p.category == category)
            .collect()
    }

    /// Parse configuration from a TOML string.
    ///
    /// This is useful for testing and fuzzing.
    pub fn from_toml_str(content: &str) -> Result<Self> {
        let config: Self = toml::from_str(content)
            .map_err(|e| Error::ConfigInvalid(format!("Parse error: {}", e)))?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_loads() {
        let config = Config::default();
        assert!(!config.patterns.is_empty());
        assert!(config
            .patterns
            .iter()
            .any(|p| p.category == PatternCategory::Placeholder));
        assert!(config
            .patterns
            .iter()
            .any(|p| p.category == PatternCategory::Deferral));
        assert!(config
            .patterns
            .iter()
            .any(|p| p.category == PatternCategory::Hedging));
    }

    #[test]
    fn test_severity_scores() {
        assert_eq!(Severity::Low.score(), 1);
        assert_eq!(Severity::Medium.score(), 5);
        assert_eq!(Severity::High.score(), 15);
        assert_eq!(Severity::Critical.score(), 50);
    }

    #[test]
    fn test_validate_patterns() {
        let config = Config::default();
        if let Err(e) = config.validate_patterns() {
            panic!("Pattern validation failed: {}", e);
        }
    }

    #[test]
    fn test_config_from_toml_str() {
        let toml = r#"
            exclude_patterns = ["*.test.rs"]
            file_extensions = [".py", ".rs"]
        "#;
        let config = Config::from_toml_str(toml).unwrap();
        assert_eq!(config.exclude_patterns.len(), 1);
        assert_eq!(config.file_extensions.len(), 2);
    }

    #[test]
    fn test_config_from_toml_str_invalid() {
        let toml = r#"
            exclude_patterns = ["*.test.rs"
            file_extensions = [".py"]
        "#; // Missing closing bracket
        let result = Config::from_toml_str(toml);
        assert!(result.is_err());
    }

    #[test]
    fn test_pattern_category_default() {
        let category = PatternCategory::default();
        assert_eq!(category, PatternCategory::Placeholder);
    }

    #[test]
    fn test_severity_default() {
        let severity = Severity::default();
        assert_eq!(severity, Severity::Medium);
    }

    #[test]
    fn test_severity_as_str() {
        assert_eq!(Severity::Low.as_str(), "LOW");
        assert_eq!(Severity::Medium.as_str(), "MEDIUM");
        assert_eq!(Severity::High.as_str(), "HIGH");
        assert_eq!(Severity::Critical.as_str(), "CRITICAL");
    }

    #[test]
    fn test_regex_pattern_new() {
        assert!(RegexPattern::new("(?i)test".to_string()).is_ok());
        assert!(RegexPattern::new("(?i)test(".to_string()).is_err());
    }

    #[test]
    fn test_regex_pattern_deref() {
        let pattern = RegexPattern::new("test".to_string()).unwrap();
        assert_eq!(&*pattern, "test");
    }

    #[test]
    fn test_regex_pattern_from_string() {
        let s = String::from("test");
        let pattern: RegexPattern = s.clone().try_into().unwrap();
        let back: String = pattern.into();
        assert_eq!(back, "test");
    }

    #[test]
    fn test_patterns_for_category() {
        let config = Config::default();
        let placeholder_patterns = config.patterns_for_category(&PatternCategory::Placeholder);
        assert!(!placeholder_patterns.is_empty());

        let deferral_patterns = config.patterns_for_category(&PatternCategory::Deferral);
        assert!(!deferral_patterns.is_empty());
    }

    #[test]
    fn test_load_or_default_with_none() {
        let config = Config::load_or_default(None);
        assert!(!config.patterns.is_empty());
    }

    #[test]
    fn test_load_or_default_with_empty_path() {
        let config = Config::load_or_default(Some(Path::new("/nonexistent/path.toml")));
        assert!(!config.patterns.is_empty());
    }
}
