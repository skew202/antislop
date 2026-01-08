//! CLI output and behavior tests.
//!
//! These tests catch mutations in CLI logic, default config values,
//! and output formatting that would otherwise go undetected.

use std::fs;
use std::process::Command;
use tempfile::TempDir;

/// Get the path to the antislop binary.
fn antislop_bin() -> String {
    if let Ok(exe) = std::env::var("CARGO_BIN_EXE_antislop") {
        return exe;
    }

    let path = std::path::PathBuf::from("target/debug/antislop");
    if !path.exists() {
        let status = Command::new("cargo")
            .args(["build", "--quiet", "--bin", "antislop"])
            .status()
            .expect("Failed to invoke cargo build");
        assert!(status.success(), "Failed to build antislop for CLI tests");
    }
    path.to_string_lossy().to_string()
}

#[test]
fn test_list_languages_includes_all_supported() {
    let output = Command::new(antislop_bin())
        .arg("--list-languages")
        .output()
        .unwrap();

    let text = String::from_utf8_lossy(&output.stdout);

    // Verify major supported languages are listed in CLI output
    assert!(text.contains("Python"), "Should list Python");
    assert!(text.contains("Rust"), "Should list Rust");
    assert!(text.contains("JavaScript"), "Should list JavaScript");
    assert!(text.contains("TypeScript"), "Should list TypeScript");
    assert!(text.contains("Go"), "Should list Go");
    assert!(text.contains("Java"), "Should list Java");
    assert!(text.contains("Ruby"), "Should list Ruby");
    assert!(text.contains("C/C++"), "Should list C/C++");
    assert!(text.contains("C#"), "Should list C#");
    assert!(text.contains("PHP"), "Should list PHP");
    assert!(text.contains("Kotlin"), "Should list Kotlin");
    assert!(text.contains("Swift"), "Should list Swift");
    assert!(text.contains("Shell"), "Should list Shell");
    assert!(text.contains("JSX"), "Should list JSX");
    assert!(text.contains("TSX"), "Should list TSX");
}

#[test]
fn test_print_config_outputs_valid_toml() {
    let output = Command::new(antislop_bin())
        .arg("--print-config")
        .output()
        .unwrap();

    let text = String::from_utf8_lossy(&output.stdout);

    // Should contain default file extensions
    assert!(text.contains(".py"), "Should include .py extension");
    assert!(text.contains(".rs"), "Should include .rs extension");
    assert!(text.contains(".js"), "Should include .js extension");

    // Should be valid TOML (basic check)
    assert!(
        text.contains("file_extensions"),
        "Should have file_extensions key"
    );
    assert!(
        text.contains("max_file_size"),
        "Should have max_file_size key"
    );
}

#[test]
fn test_sarif_severity_levels_all_present() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("test.py");

    // Use standard profile to ensure we have patterns for all severity levels
    // CRITICAL: raise NotImplementedError (core.toml - Python)
    // HIGH: XXX marker (core.toml)
    // MEDIUM: TODO: (core.toml)
    // LOW: hardcoded (antislop-standard.toml)
    fs::write(
        &file,
        r#"def test():
    # TODO: implement this (MEDIUM)
    # XXX: urgent (HIGH)
    # hardcoded value (LOW)
    raise NotImplementedError() # CRITICAL
"#,
    )
    .unwrap();

    let output = Command::new(antislop_bin())
        .arg("--format")
        .arg("sarif")
        .arg("--profile")
        .arg("antislop-standard")
        .arg(file.to_string_lossy().as_ref())
        .output()
        .unwrap();

    let text = String::from_utf8_lossy(&output.stdout);

    // Check SARIF is valid JSON
    let json: serde_json::Value = serde_json::from_str(&text).expect("SARIF should be valid JSON");

    // Verify all severity levels map to correct SARIF levels
    let run = &json["runs"][0];
    let results = run["results"].as_array().unwrap();

    let mut has_error = false;
    let mut has_warning = false;
    let mut has_note = false;

    for result in results {
        if let Some(level) = result["level"].as_str() {
            match level {
                "error" => has_error = true,
                "warning" => has_warning = true,
                "note" => has_note = true,
                _ => {}
            }
        }
    }

    // CRITICAL and HIGH should map to "error"
    // MEDIUM should map to "warning"
    // LOW should map to "note"
    assert!(
        has_error,
        "Should have error-level findings (CRITICAL/HIGH)"
    );
    assert!(has_warning, "Should have warning-level findings (MEDIUM)");
    assert!(has_note, "Should have note-level findings (LOW)");
}

#[test]
fn test_default_extensions_are_populated() {
    use antislop::Config;

    let config = Config::default();
    assert!(
        !config.file_extensions.is_empty(),
        "Default extensions should not be empty"
    );

    // Verify key extensions are present
    let exts: Vec<&str> = config.file_extensions.iter().map(|s| s.as_str()).collect();
    assert!(exts.contains(&".py"), "Should include .py");
    assert!(exts.contains(&".rs"), "Should include .rs");
    assert!(exts.contains(&".js"), "Should include .js");
    assert!(exts.contains(&".ts"), "Should include .ts");
    assert!(exts.contains(&".go"), "Should include .go");
}

#[test]
fn test_default_max_file_size_is_reasonable() {
    use antislop::Config;

    let config = Config::default();
    // Default should be at least 100KB to be useful
    assert!(
        config.max_file_size_kb >= 100,
        "Default max file size should be at least 100KB"
    );
    // But not unreasonably large
    assert!(
        config.max_file_size_kb <= 100000,
        "Default max file size should be under 100GB"
    );
}

#[test]
fn test_scanner_with_different_extensions() {
    use antislop::Scanner;

    let config = antislop::Config::default();
    let scanner = Scanner::new(config.patterns).unwrap();

    // Test that scanner works with different file extensions
    let test_cases = vec![
        ("test.py", "# TODO: python"),
        ("test.rs", "// TODO: rust"),
        ("test.js", "// TODO: javascript"),
        ("test.go", "// TODO: go"),
        ("test.java", "// TODO: java"),
        ("test.rb", "# TODO: ruby"),
    ];

    for (filename, code) in test_cases {
        let result = scanner.scan_file(filename, code);
        assert!(
            !result.findings.is_empty(),
            "Should detect TODO in {}",
            filename
        );
    }
}

#[test]
fn test_config_load_or_default_path_handling() {
    use antislop::Config;
    use std::path::Path;

    // Test with None
    let config1 = Config::load_or_default(None);
    assert!(
        !config1.patterns.is_empty(),
        "Default config should have patterns"
    );

    // Test with non-existent path
    let config2 = Config::load_or_default(Some(Path::new("/nonexistent/path.toml")));
    assert!(
        !config2.patterns.is_empty(),
        "Should fall back to default for missing file"
    );
}

#[test]
fn test_config_validate_patterns_actually_validates() {
    use antislop::Config;

    let config = Config::default();
    // validate_patterns should succeed for default config
    assert!(
        config.validate_patterns().is_ok(),
        "Default patterns should be valid"
    );
}

#[test]
fn test_patterns_for_category_filtering() {
    use antislop::{Config, PatternCategory};

    let config = Config::default();

    // Test that patterns_for_category actually filters
    let placeholder = config.patterns_for_category(&PatternCategory::Placeholder);
    let stub = config.patterns_for_category(&PatternCategory::Stub);

    // Should only return patterns for the requested category
    for p in placeholder {
        assert_eq!(p.category, PatternCategory::Placeholder);
    }

    for p in stub {
        assert_eq!(p.category, PatternCategory::Stub);
    }
}

#[test]
fn test_hygiene_survey_flag() {
    // Run hygiene survey on the project root
    let output = Command::new(antislop_bin())
        .arg("--hygiene-survey")
        .arg(".")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "Hygiene survey should complete successfully"
    );

    let text = String::from_utf8_lossy(&output.stdout);

    // Should contain survey sections
    assert!(
        text.contains("CODE") && text.contains("HYGIENE") && text.contains("SURVEY"),
        "Should show survey header: {}",
        text
    );
    assert!(
        text.contains("PROJECT DETECTION"),
        "Should show project detection section"
    );
    assert!(
        text.contains("LINTERS") || text.contains("FORMATTERS"),
        "Should show linters/formatters section"
    );
    assert!(
        text.contains("CI/CD") || text.contains("PIPELINES"),
        "Should show CI/CD section"
    );
    assert!(
        text.contains("RECOMMENDATIONS"),
        "Should show recommendations section"
    );
}
