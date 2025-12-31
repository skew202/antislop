//! Regex-based comment extraction fallback.
//!
//! This module provides comment extraction for languages without
//! tree-sitter support or when tree-sitter is disabled.

use crate::detector::Comment;
use regex::Regex;

/// Regex-based comment extractor.
#[derive(Clone)]
pub struct RegexExtractor {
    /// Line comment patterns.
    line_comments: Vec<Regex>,
    /// Block comment patterns (open, close).
    block_comments: Vec<(Regex, Regex)>,
}

impl RegexExtractor {
    /// Create a new regex-based extractor.
    pub fn new() -> Self {
        Self {
            line_comments: vec![
                Regex::new(r"//.*").unwrap(),
                Regex::new(r"#.*").unwrap(),
                Regex::new(r"--.*").unwrap(),
                Regex::new(r"%.*").unwrap(),
                Regex::new(r";.*").unwrap(),
            ],
            block_comments: vec![
                (Regex::new(r"/\*").unwrap(), Regex::new(r"\*/").unwrap()),
                (Regex::new(r#"""""#).unwrap(), Regex::new(r#"""""#).unwrap()),
                (Regex::new(r"'''").unwrap(), Regex::new(r"'''").unwrap()),
                (Regex::new(r"<!--").unwrap(), Regex::new(r"-->").unwrap()),
            ],
        }
    }

    /// Extract all comments from source code.
    pub fn extract(&self, source: &str) -> Vec<Comment> {
        let mut comments = Vec::new();
        let lines: Vec<&str> = source.lines().collect();

        for (idx, line) in lines.iter().enumerate() {
            // Extract line comments
            for regex in &self.line_comments {
                if let Some(mat) = regex.find(line) {
                    let content = mat.as_str()
                        .trim_start_matches(&['/', '#', '-', '%', ';', '"', '\''][..])
                        .trim_start_matches(['/', '"', '\''])
                        .trim();

                    if !content.is_empty() {
                        comments.push(Comment {
                            line: idx + 1,
                            column: mat.start() + 1,
                            content: content.to_string(),
                        });
                    }
                }
            }
        }

        // Handle block comments that span multiple lines
        self.extract_block_comments(source, &mut comments);

        comments
    }

    /// Extract block comments (multi-line).
    fn extract_block_comments(&self, source: &str, comments: &mut Vec<Comment>) {
        let lines: Vec<&str> = source.lines().collect();
        let mut in_block: Option<(usize, usize)> = None; // (start_line, start_col)

        for (idx, line) in lines.iter().enumerate() {
            if let Some((start_line, _)) = in_block {
                // Check for block end
                for (_, end_regex) in &self.block_comments {
                    if let Some(mat) = end_regex.find(line) {
                        let _end_col = mat.start();
                        let content: String = lines[start_line..=idx]
                            .join("\n")
                            .trim()
                            .to_string();

                        if !content.is_empty() {
                            comments.push(Comment {
                                line: start_line + 1,
                                column: 1,
                                content,
                            });
                        }
                        in_block = None;
                        break;
                    }
                }
            } else {
                // Check for block start
                for (start_regex, _) in &self.block_comments {
                    if let Some(mat) = start_regex.find(line) {
                        in_block = Some((idx, mat.start()));
                        break;
                    }
                }
            }
        }
    }
}

impl Default for RegexExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_line_comments() {
        let extractor = RegexExtractor::new();
        let code = r#"
fn main() {
    // TODO: implement this
    let x = 42;
    # Another comment
}
"#;
        let comments = extractor.extract(code);
        assert_eq!(comments.len(), 2);
        assert!(comments[0].content.contains("TODO"));
        assert!(comments[1].content.contains("Another"));
    }

    #[test]
    fn test_extract_python_comments() {
        let extractor = RegexExtractor::new();
        let code = r#"
def foo():
    # TODO: implement
    pass
"#;
        let comments = extractor.extract(code);
        assert_eq!(comments.len(), 1);
        assert!(comments[0].content.contains("TODO"));
    }

    #[test]
    fn test_extract_shell_comments() {
        let extractor = RegexExtractor::new();
        let code = "#!/bin/bash\n# TODO: fix this\necho 'hello'";
        let comments = extractor.extract(code);
        assert!(comments.len() >= 1);
        assert!(comments.iter().any(|c| c.content.contains("TODO")));
    }
}
