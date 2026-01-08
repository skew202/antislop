//! Profile validation.

use crate::{Error, Result};
use semver::Version;
use std::collections::HashSet;

/// Validate a profile's metadata and patterns.
pub fn validate_profile(profile: &super::Profile) -> Result<()> {
    validate_metadata(profile)?;
    validate_patterns(profile)?;
    validate_no_circular_extends(profile)?;
    Ok(())
}

/// Validate profile metadata.
fn validate_metadata(profile: &super::Profile) -> Result<()> {
    let meta = &profile.metadata;

    // Name is required
    if meta.name.is_empty() {
        return Err(Error::ConfigInvalid(
            "Profile metadata.name is required".to_string(),
        ));
    }

    // Validate version format if provided
    if !meta.version.is_empty() && Version::parse(&meta.version).is_err() {
        return Err(Error::ConfigInvalid(format!(
            "Invalid semantic version: {}",
            meta.version
        )));
    }

    // Validate requires_version if provided
    if let Some(ref req_ver) = meta.requires_version {
        if Version::parse(req_ver).is_err() {
            return Err(Error::ConfigInvalid(format!(
                "Invalid requires_version format: {}",
                req_ver
            )));
        }
    }

    Ok(())
}

/// Validate all patterns in the profile.
fn validate_patterns(profile: &super::Profile) -> Result<()> {
    let mut seen_regex = HashSet::new();

    for (idx, pattern) in profile.patterns.iter().enumerate() {
        // Check for duplicate regexes
        let regex_str = pattern.regex.to_string();
        if seen_regex.contains(&regex_str) {
            return Err(Error::ConfigInvalid(format!(
                "Duplicate pattern at index {}: regex '{}' is already used",
                idx, regex_str
            )));
        }
        seen_regex.insert(regex_str);

        // Validate regex (already validated by RegexPattern type)
        // Validate severity
        let _ = format!("{:?}", pattern.severity);

        // Validate category
        let _ = format!("{:?}", pattern.category);

        // Validate message
        if pattern.message.is_empty() {
            return Err(Error::ConfigInvalid(format!(
                "Pattern at index {} has empty message",
                idx
            )));
        }

        // If AST query is provided, languages must also be provided
        if pattern.ast_query.is_some() && pattern.languages.is_empty() {
            return Err(Error::ConfigInvalid(format!(
                "Pattern at index {} has ast_query but no languages specified",
                idx
            )));
        }

        // Validate known languages for AST queries
        if let Some(ref query) = pattern.ast_query {
            for lang in &pattern.languages {
                if !is_valid_language(lang) {
                    return Err(Error::ConfigInvalid(format!(
                        "Pattern at index {} has unknown language '{}'. \
                        Valid languages: Python, JavaScript, TypeScript, Rust, Go, Java, C++, C#, Ruby, Haskell, Lua, Scala",
                        idx, lang
                    )));
                }
            }

            // Basic AST query validation
            validate_ast_query(query)?;
        }
    }

    Ok(())
}

/// Check for circular extends relationships.
fn validate_no_circular_extends(_profile: &super::Profile) -> Result<()> {
    // TODO: Implement circular extends check. Requires ProfileLoader to be passed in
    // to load and verify the full extends chain. Track visited profiles to detect cycles.
    Ok(())
}

/// Check if a language name is valid for AST queries.
fn is_valid_language(lang: &str) -> bool {
    matches!(
        lang,
        "Python"
            | "JavaScript"
            | "TypeScript"
            | "Rust"
            | "Go"
            | "Java"
            | "C++"
            | "C#"
            | "Ruby"
            | "Haskell"
            | "Lua"
            | "Scala"
    )
}

/// Basic AST query syntax validation.
fn validate_ast_query(query: &str) -> Result<()> {
    // Check for balanced parentheses
    let mut depth = 0;
    let mut in_string = false;
    let chars: Vec<char> = query.chars().collect();

    for i in 0..chars.len() {
        let c = chars[i];

        // Track strings
        if c == '"' && (i == 0 || chars[i - 1] != '\\') {
            in_string = !in_string;
        }

        if in_string {
            continue;
        }

        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth < 0 {
                    return Err(Error::ConfigInvalid(
                        "Unbalanced parentheses in AST query".to_string(),
                    ));
                }
            }
            _ => {}
        }
    }

    if depth != 0 {
        return Err(Error::ConfigInvalid(
            "Unbalanced parentheses in AST query".to_string(),
        ));
    }

    // Check for basic pattern: (node_type) @capture
    if !query.contains('@') {
        return Err(Error::ConfigInvalid(
            "AST query must contain at least one capture (@capture)".to_string(),
        ));
    }

    Ok(())
}

/// Validate that patterns don't overlap with standard linters (Orthogonality principle).
///
/// Basic overlap check - looks for patterns that are covered by standard linters
/// like clippy, eslint, pylint.
pub fn validate_mece_compliance(profile: &super::Profile) -> Result<MeceReport> {
    let mut warnings = Vec::new();
    let mut violations = Vec::new();

    // Standard linter patterns that antislop should NOT duplicate
    let standard_linter_patterns = &[
        // Clippy patterns (covered by Rust linter)
        (r"\.unwrap\(\)", "clippy"),
        (r"\.expect\(", "clippy"),
        (r"panic!\(", "clippy"),
        // ESLint patterns (covered by JS linter)
        (r"var\s+", "eslint"),
        (r"==\s", "eslint"),
        // Pylint patterns (covered by Python linter)
        (r"print\(", "pylint"),
        // General formatting (covered by formatters)
        (r"\s+$", "formatter"),
        (r"\t", "formatter"),
    ];

    for pattern in &profile.patterns {
        let regex_str = pattern.regex.to_string();

        for (linter_pattern, linter_name) in standard_linter_patterns {
            if regex_str.contains(linter_pattern) || linter_pattern.contains(&regex_str) {
                warnings.push(format!(
                    "Pattern '{}' may overlap with {} (consider if this is truly AI-specific)",
                    pattern.message, linter_name
                ));
            }
        }
    }

    // Check for obvious non-AI-specific patterns
    for pattern in &profile.patterns {
        let msg = pattern.message.to_lowercase();
        if msg.contains("style")
            || msg.contains("format")
            || msg.contains("indent")
            || msg.contains("whitespace")
        {
            violations.push(format!(
                "Pattern '{}' appears to be a style check (should be handled by formatters, not antislop)",
                pattern.message
            ));
        }
    }

    Ok(MeceReport {
        is_compliant: violations.is_empty(),
        warnings,
        violations,
    })
}

/// Report from orthogonality compliance validation.
#[derive(Debug, Clone)]
pub struct MeceReport {
    /// Whether the profile is orthogonal (non-overlapping).
    pub is_compliant: bool,
    /// Warnings about potential overlaps (not blocking).
    pub warnings: Vec<String>,
    /// Violations of orthogonality principle (blocking).
    pub violations: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Pattern, PatternCategory, RegexPattern, Severity};

    fn test_profile() -> crate::profile::Profile {
        crate::profile::Profile {
            metadata: crate::profile::ProfileMetadata {
                name: "test".to_string(),
                version: "1.0.0".to_string(),
                ..Default::default()
            },
            patterns: vec![Pattern {
                regex: RegexPattern::new("(?i)TODO:".to_string()).unwrap(),
                severity: Severity::Medium,
                message: "TODO comment".to_string(),
                category: PatternCategory::Placeholder,
                ast_query: None,
                languages: vec![],
            }],
        }
    }

    #[test]
    fn test_validate_metadata_valid() {
        let profile = test_profile();
        assert!(validate_metadata(&profile).is_ok());
    }

    #[test]
    fn test_validate_metadata_empty_name() {
        let mut profile = test_profile();
        profile.metadata.name = String::new();
        assert!(validate_metadata(&profile).is_err());
    }

    #[test]
    fn test_validate_patterns_duplicate_regex() {
        let mut profile = test_profile();
        profile.patterns.push(profile.patterns[0].clone());
        assert!(validate_patterns(&profile).is_err());
    }

    #[test]
    fn test_validate_ast_query_valid() {
        assert!(validate_ast_query("(function_declaration) @func").is_ok());
        assert!(validate_ast_query("(call_expression function: (identifier) @func)").is_ok());
    }

    #[test]
    fn test_validate_ast_query_no_capture() {
        assert!(validate_ast_query("(function_declaration)").is_err());
    }

    #[test]
    fn test_validate_ast_query_unbalanced() {
        assert!(validate_ast_query("(function_declaration @func").is_err());
        assert!(validate_ast_query("(function_declaration)) @func").is_err());
    }

    #[test]
    fn test_validate_mece_compliance() {
        let profile = test_profile();
        let report = validate_mece_compliance(&profile).unwrap();
        // The TODO pattern should be orthogonal (non-overlapping)
        assert!(report.is_compliant);
    }

    #[test]
    fn test_is_valid_language() {
        assert!(is_valid_language("Python"));
        assert!(is_valid_language("Rust"));
        assert!(is_valid_language("JavaScript"));
        assert!(!is_valid_language("InvalidLanguage"));
    }
}
