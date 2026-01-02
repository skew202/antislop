//! Slop detection engine.
//!
//! This module provides the core scanning functionality, extracting comments
//! and matching against slop patterns.

mod patterns;
mod regex_fallback;

#[cfg(feature = "tree-sitter")]
mod tree_sitter;

pub use patterns::{CompiledPattern, PatternRegistry};
pub use regex_fallback::RegexExtractor;

use crate::config::{Pattern, PatternCategory, Severity};
use crate::Result;
use std::collections::HashMap;
use std::path::Path;

/// A comment extracted from source code.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Comment {
    /// Line number (1-indexed).
    pub line: usize,
    /// Column number (1-indexed).
    pub column: usize,
    /// The comment text content.
    pub content: String,
}

/// A single slop finding.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Finding {
    /// File path.
    pub file: String,
    /// Line number (1-indexed).
    pub line: usize,
    /// Column number (1-indexed).
    pub column: usize,
    /// Severity level.
    pub severity: Severity,
    /// Pattern category.
    pub category: PatternCategory,
    /// Human-readable message.
    pub message: String,
    /// The matched text.
    pub match_text: String,
    /// The regex pattern that matched.
    pub pattern_regex: String,
}

/// Result of scanning a single file.
#[derive(Debug, Clone, serde::Serialize)]
pub struct FileScanResult {
    /// File path.
    pub path: String,
    /// All findings in this file.
    pub findings: Vec<Finding>,
    /// Total slop score for this file.
    pub score: u32,
}

/// Summary of a scan operation.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScanSummary {
    /// Number of files scanned.
    pub files_scanned: usize,
    /// Number of files with findings.
    pub files_with_findings: usize,
    /// Total number of findings.
    pub total_findings: usize,
    /// Total slop score across all files.
    pub total_score: u32,
    /// Findings grouped by severity.
    pub by_severity: HashMap<Severity, usize>,
    /// Findings grouped by category.
    pub by_category: HashMap<PatternCategory, usize>,
}

impl ScanSummary {
    /// Create a summary from scan results.
    pub fn new(results: &[FileScanResult]) -> Self {
        let mut summary = Self {
            files_scanned: results.len(),
            files_with_findings: 0,
            total_findings: 0,
            total_score: 0,
            by_severity: HashMap::new(),
            by_category: HashMap::new(),
        };

        for result in results {
            if !result.findings.is_empty() {
                summary.files_with_findings += 1;
            }
            summary.total_findings += result.findings.len();
            summary.total_score += result.score;

            for finding in &result.findings {
                *summary
                    .by_severity
                    .entry(finding.severity.clone())
                    .or_insert(0) += 1;
                *summary
                    .by_category
                    .entry(finding.category.clone())
                    .or_insert(0) += 1;
            }
        }

        summary
    }
}

/// Language detection strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    /// Python source.
    Python,
    /// JavaScript.
    JavaScript,
    /// TypeScript.
    TypeScript,
    /// JSX (React).
    Jsx,
    /// TSX (React TypeScript).
    Tsx,
    /// Rust.
    Rust,
    /// Go.
    Go,
    /// Java.
    Java,
    /// Kotlin.
    Kotlin,
    /// C/C++.
    CCpp,
    /// C#.
    CSharp,
    /// Ruby.
    Ruby,
    /// PHP.
    Php,
    /// Swift.
    Swift,
    /// Haskell.
    Haskell,
    /// Lua.
    Lua,
    /// Perl.
    Perl,
    /// R.
    R,
    /// Scala.
    Scala,
    /// Shell scripts.
    Shell,
    /// Unknown language.
    Unknown,
}

impl Language {
    /// Detect language from file extension.
    pub fn from_path(path: &Path) -> Self {
        path.extension()
            .and_then(|e| e.to_str())
            .map(|ext| match ext {
                "py" => Language::Python,
                "js" | "mjs" | "cjs" => Language::JavaScript,
                "ts" => Language::TypeScript,
                "jsx" => Language::Jsx,
                "tsx" => Language::Tsx,
                "rs" => Language::Rust,
                "go" => Language::Go,
                "java" => Language::Java,
                "kt" | "kts" => Language::Kotlin,
                "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" => Language::CCpp,
                "cs" => Language::CSharp,
                "rb" => Language::Ruby,
                "php" => Language::Php,
                "swift" => Language::Swift,
                "hs" => Language::Haskell,
                "lua" => Language::Lua,
                "pl" | "pm" => Language::Perl,
                "r" | "R" => Language::R,
                "scala" => Language::Scala,
                "sh" | "bash" | "zsh" | "fish" => Language::Shell,
                _ => Language::Unknown,
            })
            .unwrap_or(Language::Unknown)
    }

    /// Returns true if tree-sitter supports this language.
    pub fn has_tree_sitter(self) -> bool {
        match self {
            #[cfg(feature = "python")]
            Language::Python => true,
            #[cfg(feature = "javascript")]
            Language::JavaScript | Language::Jsx => true,
            #[cfg(feature = "typescript")]
            Language::TypeScript | Language::Tsx => true,
            #[cfg(feature = "rust")]
            Language::Rust => true,
            #[cfg(feature = "go")]
            Language::Go => true,
            #[cfg(feature = "java")]
            Language::Java => true,
            #[cfg(feature = "cpp")]
            Language::CCpp => true,
            #[cfg(feature = "c-sharp")]
            Language::CSharp => true,
            #[cfg(feature = "php")]
            Language::Php => true,
            #[cfg(feature = "ruby")]
            Language::Ruby => true,
            #[cfg(feature = "haskell")]
            Language::Haskell => true,
            #[cfg(feature = "lua")]
            Language::Lua => true,
            #[cfg(feature = "scala")]
            Language::Scala => true,
            _ => false,
        }
    }

    /// Returns true if tree-sitter supports this language.
    #[cfg(not(feature = "tree-sitter"))]
    pub fn has_tree_sitter(self) -> bool {
        false
    }
}

/// Comment extractor trait.
pub trait CommentExtractor {
    /// Extract all comments from the given source code.
    fn extract(&self, source: &str) -> Vec<Comment>;
}

/// The main scanner.
pub struct Scanner {
    registry: PatternRegistry,
}

impl Scanner {
    /// Create a new scanner with the given patterns.
    pub fn new(patterns: Vec<Pattern>) -> Result<Self> {
        let registry = PatternRegistry::new(patterns)?;
        Ok(Self { registry })
    }

    /// Scan a single file.
    pub fn scan_file(&self, path: &str, content: &str) -> FileScanResult {
        let lang = Language::from_path(Path::new(path));
        let mut comment_findings = self.findings_from_comments(path, lang, content);

        // Also run AST-level detection if available
        #[cfg(feature = "tree-sitter")]
        if lang.has_tree_sitter() {
            if let Some(mut extractor) = self::tree_sitter::get_extractor(lang) {
                // Collect pattern references for AST detection
                let patterns: Vec<&Pattern> =
                    self.registry.patterns.iter().map(|p| &p.pattern).collect();
                // Convert Vec<&Pattern> to a slice that lives long enough
                let pattern_refs: Vec<Pattern> = patterns.iter().map(|p| (**p).clone()).collect();
                let ast_findings = extractor.extract_ast_findings(content, &pattern_refs);

                // Set file path and add to results
                for mut finding in ast_findings {
                    finding.file = path.to_string();
                    comment_findings.score += finding.severity.score();
                    comment_findings.findings.push(finding);
                }
            }
        }

        comment_findings
    }

    /// Extract comments using the best available method.
    fn extract_comments(&self, lang: Language, source: &str) -> Vec<Comment> {
        #[cfg(feature = "tree-sitter")]
        if lang.has_tree_sitter() {
            if let Some(mut extractor) = self::tree_sitter::get_extractor(lang) {
                return extractor.extract(source);
            }
        }

        // Fallback to regex-based extraction
        RegexExtractor::new().extract(source)
    }

    /// Convert comments to findings by matching patterns.
    fn findings_from_comments(&self, path: &str, lang: Language, source: &str) -> FileScanResult {
        let mut findings = Vec::new();
        let mut total_score = 0u32;

        let comments = self.extract_comments(lang, source);

        for comment in &comments {
            for pattern in &self.registry.patterns {
                // Skip AST-only patterns for comment-based matching
                if pattern.pattern.ast_query.is_some() {
                    continue;
                }

                if let Some(regex) = &pattern.compiled {
                    if let Some(mat) = regex.find(&comment.content) {
                        let severity = pattern.pattern.severity.clone();
                        total_score += severity.score();

                        findings.push(Finding {
                            file: path.to_string(),
                            line: comment.line,
                            column: comment.column + mat.start(),
                            severity,
                            category: pattern.pattern.category.clone(),
                            message: pattern.pattern.message.clone(),
                            match_text: mat.as_str().to_string(),
                            pattern_regex: pattern.pattern.regex.to_string(),
                        });
                    }
                }
            }
        }

        FileScanResult {
            path: path.to_string(),
            findings,
            score: total_score,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::RegexPattern;

    fn test_patterns() -> Vec<Pattern> {
        vec![
            Pattern {
                regex: RegexPattern::new("(?i)TODO:".to_string()).unwrap(),
                severity: Severity::Medium,
                message: "Placeholder comment found".to_string(),
                category: PatternCategory::Placeholder,
                ast_query: None,
                languages: vec![],
            },
            Pattern {
                regex: RegexPattern::new("(?i)for now".to_string()).unwrap(),
                severity: Severity::Low,
                message: "Deferral phrase detected".to_string(),
                category: PatternCategory::Deferral,
                ast_query: None,
                languages: vec![],
            },
        ]
    }

    #[test]
    fn test_scan_file_findings() {
        let scanner = Scanner::new(test_patterns()).unwrap();
        let code = r#"
# TODO: implement this later
# This is fine
# for now we'll do it this way
"#;
        let result = scanner.scan_file("test.py", code);
        assert_eq!(result.findings.len(), 2);
        assert_eq!(result.findings[0].category, PatternCategory::Placeholder);
        assert_eq!(result.findings[1].category, PatternCategory::Deferral);
    }

    #[test]
    fn test_score_calculation() {
        let scanner = Scanner::new(test_patterns()).unwrap();
        let code = "# TODO: fix this # for now we do this";
        let result = scanner.scan_file("test.py", code);
        assert_eq!(result.score, 6);
    }

    #[test]
    fn test_language_detection() {
        assert_eq!(Language::from_path(Path::new("test.py")), Language::Python);
        assert_eq!(Language::from_path(Path::new("test.rs")), Language::Rust);
        assert_eq!(
            Language::from_path(Path::new("test.js")),
            Language::JavaScript
        );
        assert_eq!(Language::from_path(Path::new("test.tsx")), Language::Tsx);
        assert_eq!(
            Language::from_path(Path::new("test.xyz")),
            Language::Unknown
        );
    }

    #[test]
    fn test_language_detection_all_types() {
        // Test more file extensions
        assert_eq!(
            Language::from_path(Path::new("test.ts")),
            Language::TypeScript
        );
        assert_eq!(Language::from_path(Path::new("test.jsx")), Language::Jsx);
        assert_eq!(Language::from_path(Path::new("test.go")), Language::Go);
        assert_eq!(Language::from_path(Path::new("test.java")), Language::Java);
        assert_eq!(Language::from_path(Path::new("test.kt")), Language::Kotlin);
        assert_eq!(Language::from_path(Path::new("test.kts")), Language::Kotlin);
        assert_eq!(Language::from_path(Path::new("test.c")), Language::CCpp);
        assert_eq!(Language::from_path(Path::new("test.cpp")), Language::CCpp);
        assert_eq!(Language::from_path(Path::new("test.cs")), Language::CSharp);
        assert_eq!(Language::from_path(Path::new("test.rb")), Language::Ruby);
        assert_eq!(Language::from_path(Path::new("test.php")), Language::Php);
        assert_eq!(
            Language::from_path(Path::new("test.swift")),
            Language::Swift
        );
        assert_eq!(Language::from_path(Path::new("test.hs")), Language::Haskell);
        assert_eq!(Language::from_path(Path::new("test.lua")), Language::Lua);
        assert_eq!(Language::from_path(Path::new("test.pl")), Language::Perl);
        assert_eq!(Language::from_path(Path::new("test.pm")), Language::Perl);
        assert_eq!(
            Language::from_path(Path::new("test.scala")),
            Language::Scala
        );
        assert_eq!(Language::from_path(Path::new("test.sh")), Language::Shell);
        assert_eq!(Language::from_path(Path::new("test.bash")), Language::Shell);
        assert_eq!(Language::from_path(Path::new("test.zsh")), Language::Shell);
        assert_eq!(Language::from_path(Path::new("test.fish")), Language::Shell);
    }

    #[test]
    fn test_language_from_path_no_extension() {
        // Test paths without extension
        assert_eq!(
            Language::from_path(Path::new("Makefile")),
            Language::Unknown
        );
        assert_eq!(
            Language::from_path(Path::new(".gitignore")),
            Language::Unknown
        );
        assert_eq!(Language::from_path(Path::new("test")), Language::Unknown);
    }

    #[test]
    fn test_comment_struct() {
        let comment = Comment {
            line: 10,
            column: 5,
            content: "TODO: implement this".to_string(),
        };
        assert_eq!(comment.line, 10);
        assert_eq!(comment.column, 5);
        assert_eq!(comment.content, "TODO: implement this");
    }

    #[test]
    fn test_finding_struct() {
        let finding = Finding {
            file: "test.py".to_string(),
            line: 10,
            column: 5,
            severity: Severity::Medium,
            category: PatternCategory::Placeholder,
            message: "TODO comment found".to_string(),
            match_text: "TODO".to_string(),
            pattern_regex: "(?i)todo".to_string(),
        };
        assert_eq!(finding.file, "test.py");
        assert_eq!(finding.line, 10);
        assert_eq!(finding.severity, Severity::Medium);
        assert_eq!(finding.category, PatternCategory::Placeholder);
    }

    #[test]
    fn test_file_scan_result_struct() {
        let result = FileScanResult {
            path: "test.py".to_string(),
            findings: vec![],
            score: 0,
        };
        assert_eq!(result.path, "test.py");
        assert!(result.findings.is_empty());
        assert_eq!(result.score, 0);
    }

    #[test]
    fn test_scan_summary_new_empty() {
        let results = vec![];
        let summary = ScanSummary::new(&results);
        assert_eq!(summary.files_scanned, 0);
        assert_eq!(summary.files_with_findings, 0);
        assert_eq!(summary.total_findings, 0);
        assert_eq!(summary.total_score, 0);
    }

    #[test]
    fn test_scan_summary_new_with_results() {
        let results = vec![FileScanResult {
            path: "test.py".to_string(),
            findings: vec![Finding {
                file: "test.py".to_string(),
                line: 1,
                column: 1,
                severity: Severity::Medium,
                category: PatternCategory::Placeholder,
                message: "TODO".to_string(),
                match_text: "TODO".to_string(),
                pattern_regex: "(?i)todo".to_string(),
            }],
            score: 5,
        }];
        let summary = ScanSummary::new(&results);
        assert_eq!(summary.files_scanned, 1);
        assert_eq!(summary.files_with_findings, 1);
        assert_eq!(summary.total_findings, 1);
        assert_eq!(summary.total_score, 5);
        assert_eq!(*summary.by_severity.get(&Severity::Medium).unwrap(), 1);
        assert_eq!(
            *summary
                .by_category
                .get(&PatternCategory::Placeholder)
                .unwrap(),
            1
        );
    }

    #[test]
    fn test_scan_summary_new_empty_results() {
        let results = vec![
            FileScanResult {
                path: "clean.py".to_string(),
                findings: vec![],
                score: 0,
            },
            FileScanResult {
                path: "sloppy.py".to_string(),
                findings: vec![],
                score: 0,
            },
        ];
        let summary = ScanSummary::new(&results);
        assert_eq!(summary.files_scanned, 2);
        assert_eq!(summary.files_with_findings, 0);
        assert_eq!(summary.total_findings, 0);
        assert_eq!(summary.total_score, 0);
    }
}
