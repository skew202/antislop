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

    #[test]
    fn test_by_severity() {
        let patterns = vec![
            Pattern {
                regex: RegexPattern::new("(?i)HIGH:".to_string()).unwrap(),
                severity: Severity::High,
                message: "HIGH".to_string(),
                category: PatternCategory::Stub,
                ast_query: None,
                languages: vec![],
            },
            Pattern {
                regex: RegexPattern::new("(?i)MEDIUM:".to_string()).unwrap(),
                severity: Severity::Medium,
                message: "MEDIUM".to_string(),
                category: PatternCategory::Stub,
                ast_query: None,
                languages: vec![],
            },
            Pattern {
                regex: RegexPattern::new("(?i)LOW:".to_string()).unwrap(),
                severity: Severity::Low,
                message: "LOW".to_string(),
                category: PatternCategory::Stub,
                ast_query: None,
                languages: vec![],
            },
        ];

        let registry = PatternRegistry::new(patterns).unwrap();
        let high_patterns = registry.by_severity(Severity::High);
        assert_eq!(high_patterns.len(), 1);
        assert_eq!(high_patterns[0].pattern.severity, Severity::High);

        let medium_patterns = registry.by_severity(Severity::Medium);
        assert_eq!(medium_patterns.len(), 1);
        assert_eq!(medium_patterns[0].pattern.severity, Severity::Medium);

        let low_patterns = registry.by_severity(Severity::Low);
        assert_eq!(low_patterns.len(), 1);
        assert_eq!(low_patterns[0].pattern.severity, Severity::Low);

        let critical_patterns = registry.by_severity(Severity::Critical);
        assert_eq!(critical_patterns.len(), 0);
    }
}
