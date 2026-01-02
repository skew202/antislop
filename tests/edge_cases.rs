//! Edge case tests for antislop.
//!
//! These tests verify behavior with unusual inputs that might cause issues.

use antislop::{config::Config, Scanner};

#[test]
fn test_empty_input() {
    let config = Config::default();
    let scanner = Scanner::new(config.patterns).expect("Scanner creation failed");

    let result = scanner.scan_file("empty.py", "");
    assert!(result.findings.is_empty());
    assert_eq!(result.score, 0);
}

#[test]
fn test_whitespace_only() {
    let config = Config::default();
    let scanner = Scanner::new(config.patterns).expect("Scanner creation failed");

    let result = scanner.scan_file("whitespace.py", "   \n\t\n   ");
    assert!(result.findings.is_empty());
    assert_eq!(result.score, 0);
}

#[test]
fn test_unicode_comments() {
    let config = Config::default();
    let scanner = Scanner::new(config.patterns).expect("Scanner creation failed");

    // Code with Unicode characters and emoji
    let code = "def foo():\n    # TODO: 实现这个功能\n    pass";

    let result = scanner.scan_file("unicode.py", code);
    // Should still detect TODO even with Unicode
    assert!(!result.findings.is_empty());
    assert!(result.findings[0].match_text.contains("TODO"));
}

#[test]
fn test_very_long_line() {
    let config = Config::default();
    let scanner = Scanner::new(config.patterns).expect("Scanner creation failed");

    let long_line = format!("# TODO: {}", "x".repeat(10000));
    let result = scanner.scan_file("long.py", &long_line);

    // Should detect TODO even in very long lines
    assert!(!result.findings.is_empty());
}

#[test]
fn test_multiline_comment_with_todo() {
    let config = Config::default();
    let scanner = Scanner::new(config.patterns).expect("Scanner creation failed");

    // TODO in a multiline comment (Python-style)
    let code = r#"
def foo():
    '''
    TODO: implement this
    '''
    pass
"#;

    let _result = scanner.scan_file("multiline.py", code);
    // Tree-sitter should extract this as a comment
    // The comment extraction may or may not catch it depending on the implementation
    // We just verify it doesn't panic
}

#[test]
fn test_no_newline_at_end() {
    let config = Config::default();
    let scanner = Scanner::new(config.patterns).expect("Scanner creation failed");

    // File without trailing newline
    let code = "# TODO: fix this";
    let result = scanner.scan_file("no_newline.py", code);

    assert!(!result.findings.is_empty());
    assert_eq!(result.findings[0].line, 1);
}

#[test]
fn test_only_newlines() {
    let config = Config::default();
    let scanner = Scanner::new(config.patterns).expect("Scanner creation failed");

    let result = scanner.scan_file("only_newlines.py", "\n\n\n\n");
    assert!(result.findings.is_empty());
}

#[test]
fn test_carriage_return_line_feeds() {
    let config = Config::default();
    let scanner = Scanner::new(config.patterns).expect("Scanner creation_failed");

    // Windows-style line endings
    let code = "# TODO: fix this\r\n# normal comment\r\n";
    let result = scanner.scan_file("crlf.py", code);

    assert!(!result.findings.is_empty());
    assert_eq!(result.findings[0].line, 1);
}

#[test]
fn test_tab_indented_todo() {
    let config = Config::default();
    let scanner = Scanner::new(config.patterns).expect("Scanner creation failed");

    // TODO with tab indentation
    let code = "def foo():\n\t\t# TODO: implement\n\t\tpass";
    let result = scanner.scan_file("tabs.py", code);

    assert!(!result.findings.is_empty());
}
