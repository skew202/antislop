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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    /// Line number (1-indexed).
    pub line: usize,
    /// Column number (1-indexed).
    pub column: usize,
    /// The comment text content.
    pub content: String,
}

/// A single slop finding.
#[derive(Debug, Clone)]
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
    #[allow(dead_code)]
    pub pattern_regex: String,
}

/// Result of scanning a single file.
#[derive(Debug, Clone)]
pub struct FileScanResult {
    /// File path.
    #[allow(dead_code)]
    pub path: String,
    /// All findings in this file.
    pub findings: Vec<Finding>,
    /// Total slop score for this file.
    pub score: u32,
}

/// Summary of a scan operation.
#[derive(Debug, Clone)]
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
                *summary.by_severity.entry(finding.severity.clone()).or_insert(0) += 1;
                *summary.by_category.entry(finding.category.clone()).or_insert(0) += 1;
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
                "sh" | "bash" | "zsh" | "fish" => Language::Shell,
                _ => Language::Unknown,
            })
            .unwrap_or(Language::Unknown)
    }

    /// Returns true if tree-sitter supports this language.
    #[cfg(feature = "tree-sitter")]
    pub fn has_tree_sitter(self) -> bool {
        matches!(
            self,
            Language::Python
                | Language::JavaScript
                | Language::Jsx
                | Language::TypeScript
                | Language::Tsx
                | Language::Rust
        )
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
                let patterns: Vec<&Pattern> = self.registry.patterns.iter().map(|p| &p.pattern).collect();
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
                            pattern_regex: pattern.pattern.regex.clone(),
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

    fn test_patterns() -> Vec<Pattern> {
        vec![
            Pattern {
                regex: "(?i)TODO:".to_string(),
                severity: Severity::Medium,
                message: "Placeholder comment found".to_string(),
                category: PatternCategory::Placeholder,
                ast_query: None,
                languages: vec![],
            },
            Pattern {
                regex: "(?i)for now".to_string(),
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
        assert_eq!(Language::from_path(Path::new("test.js")), Language::JavaScript);
        assert_eq!(Language::from_path(Path::new("test.tsx")), Language::Tsx);
        assert_eq!(Language::from_path(Path::new("test.xyz")), Language::Unknown);
    }
}
