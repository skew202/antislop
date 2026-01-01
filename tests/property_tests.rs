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
}
