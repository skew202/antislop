//! Configuration loading and management.
//!
//! Antislop uses layered configuration: built-in defaults → config file → CLI overrides.

use crate::{Error, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const DEFAULT_CONFIG_TOML: &str = include_str!("../config/default.toml");

/// Severity level for a slop finding.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// Minor issue, worth addressing but not urgent.
    Low,
    /// Moderate issue, should be fixed.
    #[default]
    Medium,
    /// Significant issue, fix recommended.
    High,
    /// Critical issue requiring immediate attention.
    Critical,
}

impl Severity {
    /// Returns the numeric score for this severity level.
    pub fn score(&self) -> u32 {
        match self {
            Severity::Low => 1,
            Severity::Medium => 5,
            Severity::High => 15,
            Severity::Critical => 50,
        }
    }

    /// Returns the display name for this severity.
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Low => "LOW",
            Severity::Medium => "MEDIUM",
            Severity::High => "HIGH",
            Severity::Critical => "CRITICAL",
        }
    }
}

/// Category of slop pattern.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum PatternCategory {
    /// Placeholder comments: TODO, FIXME, HACK, etc.
    #[default]
    Placeholder,
    /// Deferral language: "for now", "temporary", etc.
    Deferral,
    /// Hedging language: "hopefully", "should work", etc.
    Hedging,
    /// Stub code: empty implementations.
    Stub,
}

/// A single slop detection pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    /// Regular expression to match (case-insensitive supported with (?i)).
    pub regex: String,
    /// Severity level for matches.
    #[serde(default)]
    pub severity: Severity,
    /// Human-readable description.
    #[serde(default)]
    pub message: String,
    /// Category of slop this pattern detects.
    #[serde(default)]
    pub category: PatternCategory,
}

/// Main configuration structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Detection patterns.
    #[serde(default)]
    pub patterns: Vec<Pattern>,
    /// Glob patterns for paths to exclude.
    #[serde(default)]
    pub exclude: Vec<String>,
    /// Additional glob patterns for exclusion.
    #[serde(default)]
    pub exclude_patterns: Vec<String>,
    /// File extensions to scan.
    #[serde(default = "default_extensions")]
    pub file_extensions: Vec<String>,
    /// Maximum file size to scan in KB.
    #[serde(default = "default_max_file_size")]
    pub max_file_size_kb: u64,
}

fn default_extensions() -> Vec<String> {
    vec![
        ".rs", ".py", ".js", ".ts", ".jsx", ".tsx", ".go", ".java", ".kt",
        ".c", ".cpp", ".h", ".hpp", ".cs", ".php", ".rb", ".swift", ".dart",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect()
}

fn default_max_file_size() -> u64 {
    1024
}

impl Default for Config {
    fn default() -> Self {
        toml::from_str(DEFAULT_CONFIG_TOML).expect("default config must be valid")
    }
}

impl Config {
    /// Load configuration from a file.
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)
            .map_err(|e| Error::Config(format!("Parse error: {}", e)))?;
        Ok(config)
    }

    /// Load from path if it exists, otherwise return default.
    pub fn load_or_default(path: Option<&Path>) -> Self {
        match path {
            Some(p) if p.exists() => Self::load(p).unwrap_or_else(|e| {
                eprintln!("Warning: Failed to load config from {}: {}", p.display(), e);
                Self::default()
            }),
            _ => Self::default(),
        }
    }

    /// Validate all regex patterns in the config.
    pub fn validate_patterns(&self) -> Result<()> {
        for pattern in &self.patterns {
            Regex::new(&pattern.regex)
                .map_err(|e| Error::Regex(pattern.regex.clone(), e.to_string()))?;
        }
        Ok(())
    }

    /// Get all patterns for a specific category.
    pub fn patterns_for_category(&self, category: &PatternCategory) -> Vec<&Pattern> {
        self.patterns
            .iter()
            .filter(|p| &p.category == category)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_loads() {
        let config = Config::default();
        assert!(!config.patterns.is_empty());
        assert!(config.patterns.iter().any(|p| p.category == PatternCategory::Placeholder));
        assert!(config.patterns.iter().any(|p| p.category == PatternCategory::Deferral));
        assert!(config.patterns.iter().any(|p| p.category == PatternCategory::Hedging));
    }

    #[test]
    fn test_severity_scores() {
        assert_eq!(Severity::Low.score(), 1);
        assert_eq!(Severity::Medium.score(), 5);
        assert_eq!(Severity::High.score(), 15);
        assert_eq!(Severity::Critical.score(), 50);
    }

    #[test]
    fn test_validate_patterns() {
        let config = Config::default();
        assert!(config.validate_patterns().is_ok());
    }
}
