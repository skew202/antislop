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

    // Basic smoke test for JSON output format
    let code = "def foo():\n    # TODO: implement this\n    pass";
    let mut results = scanner.scan_file("test.py", code);

    // Normalize paths for snapshot stability
    results.path = "test.py".to_string();
    for finding in &mut results.findings {
        finding.file = "test.py".to_string();
    }

    assert_json_snapshot!("json_output", results);
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
fn test_placeholder_patterns_snapshot() {
    let config = Config::default();
    let scanner = Scanner::new(config.patterns).expect("Scanner creation failed");

    // Code with various placeholder patterns
    let placeholder_code = r#"
def process_data(data):
    # TODO: implement validation with json schema
    # FIXME: handle edge cases where data is None
    # HACK: quick workaround for now
    # XXX urgent issue here
    # NOTE: important reminder
    pass

# REVIEW: check this later
# BUG: known issue in production
# CLEANUP: technical debt
# REFACTOR: code needs improvement
"#;

    let mut results = scanner.scan_file("placeholder.py", placeholder_code);

    // Normalize paths for snapshot stability
    results.path = "placeholder.py".to_string();
    for finding in &mut results.findings {
        finding.file = "placeholder.py".to_string();
    }

    assert_json_snapshot!("placeholder_patterns", results);
}

#[test]
fn test_deferral_patterns_snapshot() {
    let config = Config::default();
    let scanner = Scanner::new(config.patterns).expect("Scanner creation failed");

    // Code with various deferral patterns
    let deferral_code = r#"
# Quick implementation for now
def quick_implement():
    # temporary workaround
    temp_var = "value"

    # naive approach - fix later
    result = brute_force_search(data)

    # WIP: in progress
    return temp_var

# This should be in production
# In production this would be a real database
db = mock_database()

# phase 1 implementation
# future work: add error handling
# coming soon: more features
def process():
    # simplif for now
    # just a shortcut for now
    pass
"#;

    let mut results = scanner.scan_file("deferral.py", deferral_code);

    // Normalize paths for snapshot stability
    results.path = "deferral.py".to_string();
    for finding in &mut results.findings {
        finding.file = "deferral.py".to_string();
    }

    assert_json_snapshot!("deferral_patterns", results);
}

#[test]
fn test_hedging_patterns_snapshot() {
    let config = Config::default();
    let scanner = Scanner::new(config.patterns).expect("Scanner creation failed");

    // Code with various hedging patterns
    let hedging_code = r#"
# We can't easily inject the reason without changing the model schema
# or adding it to metadata, but let's log it.
logger.info(f"Gear 1 Hygiene Score: {hygiene_metric.score}")

# This should work in most cases
def basic_implementation():
    # approximately correct calculation
    result = roughly_estimate(data)

    # In a real world scenario, this would be different
    # but let's just try this approach
    # let's assume this works for now
    return result

# hopefully this doesn't crash
# seems to work fine
def risky_operation():
    # might cause issues but probably fine
    # guess this is acceptable
    pass
"#;

    let mut results = scanner.scan_file("hedging.py", hedging_code);

    // Normalize paths for snapshot stability
    results.path = "hedging.py".to_string();
    for finding in &mut results.findings {
        finding.file = "hedging.py".to_string();
    }

    assert_json_snapshot!("hedging_patterns", results);
}

#[test]
fn test_stub_patterns_snapshot() {
    let config = Config::default();
    let scanner = Scanner::new(config.patterns).expect("Scanner creation failed");

    // Code with various stub patterns
    let stub_code = r#"
def not_implemented_function():
    # not implemented yet
    raise NotImplementedError

# placeholder implementation
def stub_function():
    # MOCK: returns fake data for testing
    # DUMMY: placeholder implementation
    # FAKE: simulated result
    data = fake_data()

    # hardcoded path to config
    config_path = "/etc/app/config.toml"

    # TODO: implement
    pass

# magic number without explanation
def calculate():
    return 42 * 3.14159
"#;

    let mut results = scanner.scan_file("stub.py", stub_code);

    // Normalize paths for snapshot stability
    results.path = "stub.py".to_string();
    for finding in &mut results.findings {
        finding.file = "stub.py".to_string();
    }

    assert_json_snapshot!("stub_patterns", results);
}
