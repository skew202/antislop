//! Property-based tests for antislop.
//!
//! # Error Handling in Tests
//!
//! These tests use `.unwrap()` because test failures are acceptable to panic.
//! Property tests use `prop_assert!` macros for better failure reporting.

use antislop::{
    config::{Pattern, PatternCategory, RegexPattern, Severity},
    Scanner,
};
use proptest::prelude::*;

fn get_test_scanner() -> Scanner {
    let patterns = vec![Pattern {
        regex: RegexPattern::new("TODO|FIXME|HACK".to_string()).unwrap(),
        severity: Severity::High,
        message: "Slop".to_string(),
        category: PatternCategory::Placeholder,
        ast_query: None,
        languages: vec![],
    }];
    Scanner::new(patterns).unwrap()
}

proptest! {
    #[test]
    fn test_scanner_no_crash_on_random_input(s in "\\PC*") {
        let scanner = get_test_scanner();
        // Use .txt extension to force regex fallback
        let _ = scanner.scan_file("test.txt", &s);
    }

    #[test]
    fn test_scanner_finds_injected_slop_with_fallback(
        prefix in "[a-zA-Z0-9 ]*",
        slop in "TODO|FIXME|HACK",
        suffix in "[a-zA-Z0-9 ]*"
    ) {
        let code = format!("{} {} {}", prefix, slop, suffix);
        let scanner = get_test_scanner();

        // Construct a comment line for regex fallback to pick up
        let comment_code = format!("# {}", code);
        let result = scanner.scan_file("test.txt", &comment_code);

        // We expect findings
        prop_assert!(!result.findings.is_empty());
    }

    #[test]
    fn test_scan_result_score_matches_findings(count in 0usize..10) {
        let scanner = get_test_scanner();
        let mut code = String::new();

        // Add `count` TODOs
        for i in 0..count {
            code.push_str(&format!("# TODO item {}\n", i));
        }

        let result = scanner.scan_file("test.txt", &code);

        prop_assert_eq!(result.findings.len(), count);
        // Each High severity finding has score 15
        prop_assert_eq!(result.score, count as u32 * 15);
    }

    #[test]
    fn test_finding_positions_are_valid(line_count in 1usize..50usize) {
        let scanner = get_test_scanner();
        // Create multi-line code with TODO at the end
        let mut code = String::new();
        for i in 1..=line_count {
            if i == line_count {
                code.push_str("# TODO here\n");
            } else {
                code.push_str("normal code line\n");
            }
        }

        let result = scanner.scan_file("test.txt", &code);

        if !result.findings.is_empty() {
            let finding = &result.findings[0];
            // Finding should be on line `line_count` (TODO at end)
            prop_assert_eq!(finding.line, line_count);
            // Column should be reasonable (between 1 and length of line + padding)
            prop_assert!(finding.column >= 1);
            prop_assert!(finding.column <= 20);
        }
    }

    #[test]
    fn test_multiple_slop_patterns_in_same_line(slop1 in "TODO|FIXME|HACK", slop2 in "TODO|FIXME|HACK") {
        let scanner = get_test_scanner();
        let code = format!("# {} and {}", slop1, slop2);

        let result = scanner.scan_file("test.txt", &code);

        // Should find at least one slop pattern
        prop_assert!(!result.findings.is_empty());
        // Score should be at least 15 (High severity)
        prop_assert!(result.score >= 15);
    }
}
