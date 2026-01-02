//! Snapshot tests for antislop.
//!
//! # Error Handling in Tests
//!
//! These tests use `.expect()` instead of `.unwrap()` for slightly better error messages.
//! Test failures are acceptable to panic since they indicate bugs in the code.
//!
//! # Snapshot Stability
//!
//! All snapshots normalize file paths to avoid environment-specific differences.

use antislop::{config::Config, Scanner};
use insta::assert_json_snapshot;

#[test]
fn test_json_output_snapshot() {
    let config = Config::default();
    let scanner = Scanner::new(config.patterns).expect("Scanner creation failed");

    let code = "def foo():\n    # TODO: implement this\n    pass";
    let mut results = scanner.scan_file("test.py", code);

    // Normalize paths for snapshot stability
    results.path = "test.py".to_string();
    for finding in &mut results.findings {
        finding.file = "test.py".to_string();
    }

    assert_json_snapshot!(results);
}

#[test]
fn test_multiple_findings_snapshot() {
    let config = Config::default();
    let scanner = Scanner::new(config.patterns).expect("Scanner creation failed");

    // Creative sloppy code with multiple issue types
    let sloppy_code = r#"
def process_data(data):
    # TODO: implement validation with json schema
    # FIXME: handle edge cases where data is None
    
    try:
        # for now just return the data if it looks ok
        if len(data) > 0:
            return data
    except:
        # hopefully this works and doesn't crash
        pass
        
    return None
"#;

    let mut results = scanner.scan_file("test.py", sloppy_code);

    // Normalize paths for snapshot stability
    results.path = "test.py".to_string();
    for finding in &mut results.findings {
        finding.file = "test.py".to_string();
    }

    assert_json_snapshot!("multiple_findings", results);
}

#[test]
fn test_clean_code_snapshot() {
    let config = Config::default();
    let scanner = Scanner::new(config.patterns).expect("Scanner creation failed");

    // Clean code with no slop
    let clean_code = r#"
def add(a: int, b: int) -> int:
    """Add two integers and return the result."""
    return a + b

def multiply(x: float, y: float) -> float:
    """Multiply two floats and return the result."""
    return x * y
"#;

    let mut results = scanner.scan_file("clean.py", clean_code);

    // Normalize paths for snapshot stability
    results.path = "clean.py".to_string();

    assert_json_snapshot!("clean_code", results);
}

#[test]
fn test_severity_levels_snapshot() {
    let config = Config::default();
    let scanner = Scanner::new(config.patterns).expect("Scanner creation failed");

    // Code with different severity levels
    let mixed_code = r#"
def critical_function():
    # CRITICAL: security vulnerability - fix immediately
    # HACK: this is a quick workaround
    # FIXME: refactor this later
    # TODO: implement properly
    pass
"#;

    let mut results = scanner.scan_file("severity.py", mixed_code);

    // Normalize paths for snapshot stability
    results.path = "severity.py".to_string();
    for finding in &mut results.findings {
        finding.file = "severity.py".to_string();
    }

    assert_json_snapshot!("severity_levels", results);
}

#[test]
fn test_stub_patterns_snapshot() {
    let config = Config::default();
    let scanner = Scanner::new(config.patterns).expect("Scanner creation failed");

    // Code with stub patterns
    let stub_code = r#"
def mock_data():
    # MOCK: returns fake data for testing
    # DUMMY: placeholder implementation
    data = simulate_result()
    return data
"#;

    let mut results = scanner.scan_file("stub.py", stub_code);

    // Normalize paths for snapshot stability
    results.path = "stub.py".to_string();
    for finding in &mut results.findings {
        finding.file = "stub.py".to_string();
    }

    assert_json_snapshot!("stub_patterns", results);
}
