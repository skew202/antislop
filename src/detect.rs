use crate::config::{Pattern, PatternCategory, Severity};
use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    pub line: usize,
    pub content: String,
    pub prefix_len: usize,
}

#[derive(Debug, Clone)]
pub struct Finding {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub severity: Severity,
    pub category: PatternCategory,
    pub message: String,
    pub match_text: String,
    #[allow(dead_code)]
    pub pattern_regex: String,
}

#[derive(Debug, Clone)]
pub struct FileScanResult {
    #[allow(dead_code)]
    pub path: String,
    pub findings: Vec<Finding>,
    pub score: u32,
}

#[derive(Debug, Clone)]
pub struct ScanSummary {
    pub files_scanned: usize,
    pub files_with_findings: usize,
    pub total_findings: usize,
    pub total_score: u32,
    pub by_severity: HashMap<Severity, usize>,
    pub by_category: HashMap<PatternCategory, usize>,
}

pub struct Scanner {
    compiled: Vec<(Regex, Pattern)>,
    comment_regexes: Vec<Regex>,
}

impl Scanner {
    pub fn new(patterns: Vec<Pattern>) -> Result<Self> {
        let compiled: Vec<_> = patterns
            .into_iter()
            .map(|p| {
                let regex = Regex::new(&p.regex)
                    .map_err(|e| anyhow::anyhow!("Invalid regex '{}': {}", p.regex, e))?;
                Ok::<_, anyhow::Error>((regex, p))
            })
            .collect::<Result<_, _>>()?;

        let comment_regexes = vec![
            Regex::new(r#"//.*"#)?,
            Regex::new(r"#.*")?,
            Regex::new(r"/\*.*?\*/")?,
            Regex::new(r#""""[\s\S]*?""""#)?,
            Regex::new(r"'''.*?'''")?,
            Regex::new(r"<!--.*?-->")?,
        ];

        Ok(Self {
            compiled,
            comment_regexes,
        })
    }

    pub fn scan_file(&self, path: &str, content: &str) -> FileScanResult {
        let comments = self.extract_comments(content);
        let mut findings = Vec::new();
        let mut total_score = 0u32;

        for comment in &comments {
            for (regex, pattern) in &self.compiled {
                if let Some(mat) = regex.find(&comment.content) {
                    let severity = pattern.severity.clone();
                    total_score += severity.score();

                    findings.push(Finding {
                        file: path.to_string(),
                        line: comment.line,
                        column: comment.prefix_len + mat.start() + 1,
                        severity,
                        category: pattern.category.clone(),
                        message: pattern.message.clone(),
                        match_text: mat.as_str().to_string(),
                        pattern_regex: pattern.regex.clone(),
                    });
                }
            }
        }

        FileScanResult {
            path: path.to_string(),
            findings,
            score: total_score,
        }
    }

    fn extract_comments(&self, content: &str) -> Vec<Comment> {
        let mut comments = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            for regex in &self.comment_regexes {
                if let Some(mat) = regex.find(line) {
                    let comment_text = mat.as_str().trim_start_matches(&['/', '#', '*', '<', '!', '"', '\''][..])
                        .trim_start_matches(['/', '*', '"', '\''])
                        .trim();

                    if !comment_text.is_empty() {
                        comments.push(Comment {
                            line: idx + 1,
                            content: comment_text.to_string(),
                            prefix_len: mat.start(),
                        });
                    }
                }
            }

            if trimmed.starts_with("/*") && trimmed.ends_with("*/") {
                let inner = trimmed[2..trimmed.len() - 2].trim();
                if !inner.is_empty() && !comments.iter().any(|c| c.line == idx + 1) {
                    comments.push(Comment {
                        line: idx + 1,
                        content: inner.to_string(),
                        prefix_len: line.chars().take_while(|&c| c.is_whitespace()).count(),
                    });
                }
            }
        }

        comments
    }
}

impl ScanSummary {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_patterns() -> Vec<Pattern> {
        vec![
            Pattern {
                regex: "TODO:".to_string(),
                severity: Severity::Medium,
                message: "Placeholder comment found".to_string(),
                category: PatternCategory::Placeholder,
            },
            Pattern {
                regex: "for now".to_string(),
                severity: Severity::Low,
                message: "Deferral phrase detected".to_string(),
                category: PatternCategory::Deferral,
            },
            Pattern {
                regex: "hopefully".to_string(),
                severity: Severity::Low,
                message: "Hedging language detected".to_string(),
                category: PatternCategory::Hedging,
            },
        ]
    }

    #[test]
    fn test_extract_single_line_comments() {
        let scanner = Scanner::new(test_patterns()).unwrap();
        let code = r#"
fn main() {
    // TODO: implement this
    let x = 42;
}
"#;
        let comments = scanner.extract_comments(code);
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].line, 3);
        assert!(comments[0].content.contains("TODO"));
    }

    #[test]
    fn test_scan_file_findings() {
        let scanner = Scanner::new(test_patterns()).unwrap();
        let code = r#"
// TODO: implement this later
// This is fine
// for now we'll do it this way
"#;
        let result = scanner.scan_file("test.rs", code);
        assert_eq!(result.findings.len(), 2);
        assert_eq!(result.findings[0].category, PatternCategory::Placeholder);
        assert_eq!(result.findings[1].category, PatternCategory::Deferral);
    }

    #[test]
    fn test_score_calculation() {
        let scanner = Scanner::new(test_patterns()).unwrap();
        let code = "// TODO: fix this // hopefully it works";
        let result = scanner.scan_file("test.rs", code);
        assert_eq!(result.score, 6);
    }
}
