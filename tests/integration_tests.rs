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
        // Build it first
        let status = Command::new("cargo")
            .args(["build", "--quiet"])
            .current_dir("..")
            .status()
            .expect("Failed to build antislop");
        assert!(status.success(), "Failed to build antislop for integration tests");
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

    let output = Command::new(&antislop_bin())
        .arg(file.to_string_lossy().as_ref())
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    if !text.contains("No AI slop detected") && !text.contains("Clean code") && !text.contains("✓") {
        eprintln!("Output was: {:?}", text);
    }
    assert!(text.contains("No AI slop detected") || text.contains("Clean code") || text.contains("✓"));
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

    let output = Command::new(&antislop_bin())
        .arg(file.to_string_lossy().as_ref())
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    if !text.contains("TODO") && !text.contains("placeholder") {
        eprintln!("Output was: {:?}", text);
    }
    assert!(text.contains("TODO") || text.contains("placeholder") || text.contains("MEDIUM"));
}

#[test]
fn test_help() {
    let output = Command::new(&antislop_bin())
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
    let output = Command::new(&antislop_bin())
        .arg("--version")
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    assert!(text.contains("antislop"));
}

#[test]
fn test_list_languages() {
    let output = Command::new(&antislop_bin())
        .arg("--list-languages")
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    assert!(text.contains("Python"));
    assert!(text.contains("Rust"));
    assert!(text.contains("JavaScript"));
}
