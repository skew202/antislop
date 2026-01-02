//! Common test utilities for antislop tests.
//!
//! This module provides shared helper functions to reduce duplication
//! across unit tests, integration tests, and snapshot tests.

use antislop::config::{PatternCategory, Severity};
use antislop::detector::Finding;

/// Create a test finding with the specified parameters.
///
/// This helper reduces boilerplate in tests that need to create Finding instances.
pub fn make_finding(
    file: &str,
    line: usize,
    column: usize,
    severity: Severity,
    category: PatternCategory,
    message: &str,
    match_text: &str,
) -> Finding {
    Finding {
        file: file.to_string(),
        line,
        column,
        severity,
        category,
        message: message.to_string(),
        match_text: match_text.to_string(),
        pattern_regex: "test".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_finding() {
        let finding = make_finding(
            "test.py",
            42,
            5,
            Severity::High,
            PatternCategory::Placeholder,
            "Test message",
            "TODO",
        );
        assert_eq!(finding.file, "test.py");
        assert_eq!(finding.line, 42);
        assert_eq!(finding.column, 5);
        assert_eq!(finding.severity, Severity::High);
        assert_eq!(finding.category, PatternCategory::Placeholder);
        assert_eq!(finding.message, "Test message");
        assert_eq!(finding.match_text, "TODO");
    }
}
