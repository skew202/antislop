//! Integration tests for the CLI.

use std::fs;
use std::process::Command;
use tempfile::TempDir;

/// Get the path to the antislop binary.
fn antislop_bin() -> String {
    // Use CARGO_BIN_EXE if available (set by cargo test)
    if let Ok(exe) = std::env::var("CARGO_BIN_EXE_antislop") {
        return exe;
    }

    // Fallback: look in target/debug
    let path = std::path::PathBuf::from("target/debug/antislop");
    if !path.exists() {
        // Build it first (use correct manifest path)
        let status = Command::new("cargo")
            .args(["build", "--quiet", "--bin", "antislop"])
            .status()
            .expect("Failed to invoke cargo build");
        assert!(
            status.success(),
            "Failed to build antislop for integration tests"
        );
    }
    path.to_string_lossy().to_string()
}

#[test]
fn test_clean_code() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("clean.py");
    fs::write(
        &file,
        r#"def add(a, b):
    return a + b
"#,
    )
    .unwrap();

    let output = Command::new(antislop_bin())
        .arg(file.to_string_lossy().as_ref())
        .output()
        .unwrap();

    // Clean code should exit with success
    assert!(
        output.status.success(),
        "Clean code should exit successfully: {:?}",
        output.status
    );
    // Output should indicate no findings
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(
        text.contains("✓") || text.contains("Clean") || text.contains("No AI slop"),
        "Expected clean code indicator, got: {}",
        text
    );
}

#[test]
fn test_todo_detection() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("todo.py");
    fs::write(
        &file,
        r#"def process(data):
    # TODO: implement validation
    return data
"#,
    )
    .unwrap();

    let output = Command::new(antislop_bin())
        .arg(file.to_string_lossy().as_ref())
        .output()
        .unwrap();

    let text = String::from_utf8_lossy(&output.stdout);
    // TODO detection should find "placeholder" category or "TODO" text
    assert!(
        text.contains("TODO") || text.contains("placeholder"),
        "Expected TODO detection, got: {}",
        text
    );
}

#[test]
fn test_help() {
    let output = Command::new(antislop_bin())
        .arg("--help")
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    assert!(text.contains("antislop"));
    assert!(text.contains("AI-generated code slop"));
}

#[test]
fn test_version() {
    let output = Command::new(antislop_bin())
        .arg("--version")
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    assert!(text.contains("antislop"));
}

#[test]
fn test_list_languages() {
    let output = Command::new(antislop_bin())
        .arg("--list-languages")
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    assert!(text.contains("Python"));
    assert!(text.contains("Rust"));
    assert!(text.contains("JavaScript"));
}

#[test]
fn test_nonexistent_file() {
    let output = Command::new(antislop_bin())
        .arg("nonexistent_file_definitely_does_not_exist.rs")
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("No such file")
            || stderr.contains("not found")
            || stderr.to_lowercase().contains("error")
            || stderr.contains("No files found")
    );
}

#[test]
fn test_sarif_output_flag() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("sarif_test.py");
    fs::write(&file, "def foo():\n    # TODO: fix me\n    pass").unwrap();

    let output = Command::new(antislop_bin())
        .arg("--format")
        .arg("sarif")
        .arg(file.to_string_lossy().as_ref())
        .output()
        .unwrap();

    let text = String::from_utf8_lossy(&output.stdout);
    // Validate SARIF structure by parsing as JSON
    let json: serde_json::Value =
        serde_json::from_str(&text).expect("SARIF output should be valid JSON");
    // Check SARIF version
    assert_eq!(json["version"], "2.1.0");
    // Check runs array exists and has at least one run
    assert!(json["runs"].as_array().is_some_and(|r| !r.is_empty()));
    // Check that results exist
    let run = &json["runs"][0];
    assert!(run["results"].as_array().is_some_and(|r| !r.is_empty()));
}

// Tests for mock/fake/dummy patterns
#[test]
fn test_mock_umap_detection() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("mock.rs");
    fs::write(
        &file,
        r#"fn main() {
    // MOCK UMAP
    let x = 1;
}"#,
    )
    .unwrap();

    let output = Command::new(antislop_bin())
        .arg(file.to_string_lossy().as_ref())
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    assert!(text.contains("MOCK") || text.contains("mock") || text.contains("stub"));
}

#[test]
fn test_mocking_noise_detection() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("mocking.py");
    fs::write(
        &file,
        r#"def process():
    # Mocking noise calculation for now
    return "62%"
"#,
    )
    .unwrap();

    let output = Command::new(antislop_bin())
        .arg(file.to_string_lossy().as_ref())
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    assert!(text.contains("mock") || text.contains("Mocking"));
    assert!(text.contains("for now") || text.contains("deferral"));
}

#[test]
fn test_fake_data_detection() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("fake.py");
    fs::write(
        &file,
        r#"def get_result():
    # fake result for testing
    return 42
"#,
    )
    .unwrap();

    let output = Command::new(antislop_bin())
        .arg(file.to_string_lossy().as_ref())
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    assert!(text.contains("fake") || text.contains("Fake") || text.contains("stub"));
}

#[test]
fn test_dummy_value_detection() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("dummy.rs");
    fs::write(
        &file,
        r#"fn main() {
    // DUMMY VALUE placeholder
    let x = 1;
}"#,
    )
    .unwrap();

    let output = Command::new(antislop_bin())
        .arg(file.to_string_lossy().as_ref())
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    assert!(text.contains("DUMMY") || text.contains("dummy") || text.contains("stub"));
}

#[test]
fn test_simulated_data_detection() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("simulated.py");
    fs::write(
        &file,
        r#"def calculate():
    # Simulated result for testing
    return 42
"#,
    )
    .unwrap();

    let output = Command::new(antislop_bin())
        .arg(file.to_string_lossy().as_ref())
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    assert!(text.contains("Simulated") || text.contains("simulated") || text.contains("stub"));
}

// Tests for hardcoded patterns
#[test]
fn test_hardcoded_path_detection() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("hardcoded.rs");
    fs::write(
        &file,
        r#"fn main() {
    // Hardcoded path for the prototype
    let uri = "/work/archivephoenix/processed/lance_data";
}"#,
    )
    .unwrap();

    let output = Command::new(antislop_bin())
        .arg(file.to_string_lossy().as_ref())
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    assert!(text.contains("hardcoded") || text.contains("Hardcoded") || text.contains("prototype"));
}

#[test]
fn test_hardcoded_url_detection() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("hardcoded_url.py");
    fs::write(
        &file,
        r#"def main():
    # hardcoded URL endpoint
    url = "https://api.example.com/v1"
"#,
    )
    .unwrap();

    let output = Command::new(antislop_bin())
        .arg(file.to_string_lossy().as_ref())
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    assert!(text.contains("hardcoded") || text.contains("Hardcoded") || text.contains("stub"));
}

#[test]
fn test_magic_number_detection() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("magic.rs");
    fs::write(
        &file,
        r#"fn main() {
    // magic number threshold
    let threshold = 42;
}"#,
    )
    .unwrap();

    let output = Command::new(antislop_bin())
        .arg(file.to_string_lossy().as_ref())
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    assert!(text.contains("magic") || text.contains("Magic"));
}

// Tests for production/deferral patterns
#[test]
fn test_in_production_this_would_detection() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("production.rs");
    fs::write(
        &file,
        r#"fn main() {
    // In production this would be config/env
    let uri = "/work/archivephoenix/processed/lance_data";
}"#,
    )
    .unwrap();

    let output = Command::new(antislop_bin())
        .arg(file.to_string_lossy().as_ref())
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    assert!(
        text.contains("production") || text.contains("Production") || text.contains("deferral")
    );
}

#[test]
fn test_this_should_be_detection() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("should_be.py");
    fs::write(
        &file,
        r#"def process():
    # This should be a constant
    return 42
"#,
    )
    .unwrap();

    let output = Command::new(antislop_bin())
        .arg(file.to_string_lossy().as_ref())
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    assert!(text.contains("should") || text.contains("deferral"));
}

#[test]
fn test_this_would_be_detection() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("would_be.py");
    fs::write(
        &file,
        r#"def process():
    # This would be better with a config file
    return 42
"#,
    )
    .unwrap();

    let output = Command::new(antislop_bin())
        .arg(file.to_string_lossy().as_ref())
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    assert!(text.contains("would") || text.contains("deferral"));
}

// Test that legitimate test code doesn't get flagged
#[test]
fn test_mock_test_helper_not_flagged() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("test_helper.rs");
    fs::write(
        &file,
        r#"fn main() {
    // mock_test_helper function
    // MockSuite for testing
    let x = 1;
}"#,
    )
    .unwrap();

    let output = Command::new(antislop_bin())
        .arg(file.to_string_lossy().as_ref())
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);

    // mock_test_helper contains "test" so it should NOT trigger our new mock patterns
    // The specific "mock implementation" message should not appear
    if text.contains("mock implementation") {
        panic!("False positive: mock_test_helper should not be flagged as mock implementation");
    }
}
