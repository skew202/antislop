//! Integration tests for the CLI.

use std::fs;
use tempfile::TempDir;

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

    let output = std::process::Command::new("cargo")
        .args(["run", "--", file.to_string_lossy().as_ref()])
        .current_dir(temp.path())
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    assert!(text.contains("No AI slop detected") || text.contains("Clean code"));
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

    let output = std::process::Command::new("cargo")
        .args(["run", "--", file.to_string_lossy().as_ref()])
        .current_dir(temp.path())
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    assert!(text.contains("TODO") || text.contains("placeholder"));
}

#[test]
fn test_help() {
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    assert!(text.contains("antislop"));
    assert!(text.contains("AI-generated code slop"));
}

#[test]
fn test_version() {
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "--version"])
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    assert!(text.contains("antislop"));
}

#[test]
fn test_list_languages() {
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "--list-languages"])
        .output()
        .unwrap()
        .stdout;

    let text = String::from_utf8_lossy(&output);
    assert!(text.contains("Python"));
    assert!(text.contains("Rust"));
    assert!(text.contains("JavaScript"));
}
