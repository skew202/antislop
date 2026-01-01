//! Pattern registry for slop detection.

use crate::config::{Pattern, Severity};
use crate::{Error, Result};
use regex::Regex;

/// A compiled pattern ready for matching.
pub struct CompiledPattern {
    /// Original pattern definition.
    pub pattern: Pattern,
    /// Compiled regex for matching.
    pub compiled: Option<Regex>,
}

/// Registry of slop detection patterns.
pub struct PatternRegistry {
    /// All registered patterns.
    pub patterns: Vec<CompiledPattern>,
}

impl PatternRegistry {
    /// Create a new registry from pattern definitions.
    pub fn new(patterns: Vec<Pattern>) -> Result<Self> {
        let compiled: Result<Vec<CompiledPattern>> = patterns
            .into_iter()
            .map(|p| {
                let compiled = Regex::new(&p.regex).map_err(Error::Regex)?;
                Ok(CompiledPattern {
                    compiled: Some(compiled),
                    pattern: p,
                })
            })
            .collect();

        Ok(Self {
            patterns: compiled?,
        })
    }

    /// Get all patterns.
    pub fn all(&self) -> &[CompiledPattern] {
        &self.patterns
    }

    /// Get patterns by severity.
    pub fn by_severity(&self, severity: Severity) -> Vec<&CompiledPattern> {
        self.patterns
            .iter()
            .filter(|p| p.pattern.severity == severity)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{PatternCategory, RegexPattern};

    #[test]
    fn test_registry_creation() {
        let patterns = vec![Pattern {
            regex: RegexPattern::new("(?i)TODO:".to_string()).unwrap(),
            severity: Severity::Medium,
            message: "TODO".to_string(),
            category: PatternCategory::Placeholder,
            ast_query: None,
            languages: vec![],
        }];

        let registry = PatternRegistry::new(patterns);
        assert!(registry.is_ok());
        let registry = registry.unwrap();
        assert_eq!(registry.all().len(), 1);
    }

    #[test]
    fn test_invalid_regex() {
        // RegexPattern prevents creation of invalid regex.
        // So we test RegexPattern::new failure.
        let result = RegexPattern::new("(?i)TODO:(".to_string());
        assert!(result.is_err());
    }
}
