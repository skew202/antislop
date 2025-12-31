//! Tree-sitter based comment extraction.
//!
//! Provides accurate, language-aware comment extraction using tree-sitter.

use crate::detector::Comment;

/// Get a comment extractor for the given language.
#[cfg(feature = "tree-sitter")]
pub fn get_extractor(_lang: crate::detector::Language) -> Option<TreeSitterExtractor> {
    // TODO: Add tree-sitter language support
    // For now, return None to fall back to regex-based extraction
    None
}

/// Tree-sitter based comment extractor.
#[cfg(feature = "tree-sitter")]
pub struct TreeSitterExtractor {
    _private: (),
}

#[cfg(feature = "tree-sitter")]
impl TreeSitterExtractor {
    /// Extract all comments from source code.
    pub fn extract(&self, _source: &str) -> Vec<Comment> {
        // TODO: Implement with tree-sitter grammars
        Vec::new()
    }
}

/// Dummy extractor when tree-sitter is disabled.
#[cfg(not(feature = "tree-sitter"))]
pub struct TreeSitterExtractor;

#[cfg(not(feature = "tree-sitter"))]
pub fn get_extractor(
    _lang: crate::detector::Language,
) -> Option<TreeSitterExtractor> {
    None
}

#[cfg(all(test, feature = "tree-sitter"))]
mod tests {
    use super::*;

    #[test]
    fn test_extractor_fallback() {
        let extractor = get_extractor(crate::detector::Language::Python);
        assert!(extractor.is_none());
    }
}
