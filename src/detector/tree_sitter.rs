//! Tree-sitter based comment extraction and AST slop detection.
//!
//! Provides accurate, language-aware comment extraction using tree-sitter,
//! as well as AST-level pattern matching for code slop that regex cannot detect.

use crate::config::Pattern;
use crate::detector::{Comment, Finding, Language};
use streaming_iterator::StreamingIterator;

// ...

use tree_sitter::{Node, Parser, Query, QueryCursor};

/// Get a comment extractor for the given language.
#[cfg(feature = "tree-sitter")]
pub fn get_extractor(lang: Language) -> Option<TreeSitterExtractor> {
    TreeSitterExtractor::new(lang)
}

/// Tree-sitter based comment extractor.
#[cfg(feature = "tree-sitter")]
pub struct TreeSitterExtractor {
    parser: Parser,
    language: Language,
}

#[cfg(feature = "tree-sitter")]
impl TreeSitterExtractor {
    /// Create a new extractor for the given language.
    pub fn new(lang: Language) -> Option<Self> {
        let mut parser = Parser::new();

        let language_fn = get_language_fn(lang)?;
        parser.set_language(&language_fn).ok()?;

        Some(Self {
            parser,
            language: lang,
        })
    }

    /// Extract all comments from source code.
    pub fn extract(&mut self, source: &str) -> Vec<Comment> {
        let mut comments = Vec::new();

        let tree = match self.parser.parse(source, None) {
            Some(t) => t,
            None => return comments,
        };

        extract_comments_recursive(&tree.root_node(), source, &mut comments);
        comments
    }

    /// Extract AST-level findings using tree-sitter queries.
    ///
    /// Returns findings from patterns that have `ast_query` set and apply to this language.
    pub fn extract_ast_findings(&mut self, source: &str, patterns: &[Pattern]) -> Vec<Finding> {
        let mut findings = Vec::new();

        let tree = match self.parser.parse(source, None) {
            Some(t) => t,
            None => return findings,
        };

        let lang_name = self.language_name();

        for pattern in patterns {
            // Skip patterns without AST queries or that don't apply to this language
            let query_str = match &pattern.ast_query {
                Some(q) => q,
                None => continue,
            };

            if !pattern.languages.is_empty() && !pattern.languages.contains(&lang_name.to_string())
            {
                continue;
            }

            // Get the language for query compilation
            let ts_lang = match self.parser.language() {
                Some(l) => l,
                None => continue,
            };

            // Compile and run the query
            let query = match Query::new(&ts_lang, query_str) {
                Ok(q) => q,
                Err(e) => {
                    eprintln!("Warning: Invalid tree-sitter query '{}': {}", query_str, e);
                    continue;
                }
            };

            let mut cursor = QueryCursor::new();
            let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());

            // Compile the regex for this pattern once (for validation)
            let regex = match regex::Regex::new(&pattern.regex) {
                Ok(r) => r,
                Err(_) => continue,
            };

            while let Some(mat) = matches.next() {
                for capture in mat.captures {
                    let node = capture.node;
                    let text = node.utf8_text(source.as_bytes()).unwrap_or("").to_string();

                    // Verify the regex also matches the matched text
                    // This prevents false positives from overly broad AST queries
                    if !regex.is_match(&text) {
                        continue;
                    }

                    let line = node.start_position().row + 1;
                    let column = node.start_position().column + 1;

                    findings.push(Finding {
                        file: String::new(), // Caller will set
                        line,
                        column,
                        severity: pattern.severity.clone(),
                        category: pattern.category.clone(),
                        message: pattern.message.clone(),
                        match_text: text,
                        pattern_regex: pattern.regex.to_string(),
                        source_line: None, // TODO: Extract from source
                        context_before: None,
                        context_after: None,
                    });
                }
            }
        }

        findings
    }

    fn language_name(&self) -> &'static str {
        match self.language {
            #[cfg(feature = "python")]
            Language::Python => "Python",
            #[cfg(feature = "javascript")]
            Language::JavaScript | Language::Jsx => "JavaScript",
            #[cfg(feature = "typescript")]
            Language::TypeScript | Language::Tsx => "TypeScript",
            #[cfg(feature = "rust")]
            Language::Rust => "Rust",
            #[cfg(feature = "go")]
            Language::Go => "Go",
            #[cfg(feature = "java")]
            Language::Java => "Java",
            #[cfg(feature = "cpp")]
            Language::CCpp => "C++",
            #[cfg(feature = "c-sharp")]
            Language::CSharp => "C#",
            #[cfg(feature = "php")]
            Language::Php => "PHP",
            #[cfg(feature = "ruby")]
            Language::Ruby => "Ruby",
            #[cfg(feature = "haskell")]
            Language::Haskell => "Haskell",
            #[cfg(feature = "lua")]
            Language::Lua => "Lua",
            #[cfg(feature = "scala")]
            Language::Scala => "Scala",
            _ => "Unknown",
        }
    }
}

#[cfg(feature = "tree-sitter")]
fn get_language_fn(lang: Language) -> Option<tree_sitter::Language> {
    match lang {
        #[cfg(feature = "python")]
        Language::Python => Some(tree_sitter_python::LANGUAGE.into()),
        #[cfg(feature = "javascript")]
        Language::JavaScript | Language::Jsx => Some(tree_sitter_javascript::LANGUAGE.into()),
        #[cfg(feature = "typescript")]
        Language::TypeScript | Language::Tsx => {
            Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
        }

        #[cfg(feature = "rust")]
        Language::Rust => Some(tree_sitter_rust::LANGUAGE.into()),
        #[cfg(feature = "go")]
        Language::Go => Some(tree_sitter_go::LANGUAGE.into()),
        #[cfg(feature = "java")]
        Language::Java => Some(tree_sitter_java::LANGUAGE.into()),
        #[cfg(feature = "cpp")]
        Language::CCpp => Some(tree_sitter_cpp::LANGUAGE.into()),
        #[cfg(feature = "c-sharp")]
        Language::CSharp => Some(tree_sitter_c_sharp::LANGUAGE.into()),
        #[cfg(feature = "ruby")]
        Language::Ruby => Some(tree_sitter_ruby::LANGUAGE.into()),
        #[cfg(feature = "haskell")]
        Language::Haskell => Some(tree_sitter_haskell::LANGUAGE.into()),
        #[cfg(feature = "lua")]
        Language::Lua => Some(tree_sitter_lua::LANGUAGE.into()),
        #[cfg(feature = "scala")]
        Language::Scala => Some(tree_sitter_scala::LANGUAGE.into()),
        _ => None,
    }
}

#[cfg(feature = "tree-sitter")]
fn extract_comments_recursive(node: &Node, source: &str, comments: &mut Vec<Comment>) {
    if node.kind().contains("comment") {
        let line = node.start_position().row + 1;
        let column = node.start_position().column + 1;
        let content = node.utf8_text(source.as_bytes()).unwrap_or("").to_string();

        // Strip comment markers for consistency with regex extractor
        let content = strip_comment_markers(&content, node.kind());

        comments.push(Comment {
            line,
            column,
            content,
        });
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        extract_comments_recursive(&child, source, comments);
    }
}

#[cfg(feature = "tree-sitter")]
fn strip_comment_markers(text: &str, kind: &str) -> String {
    let text = text.trim();

    // Handle various comment styles
    if kind.contains("line") {
        // Single-line comments: //, #, --, etc.
        if let Some(rest) = text.strip_prefix("//") {
            return rest.trim_start().to_string();
        }
        if let Some(rest) = text.strip_prefix('#') {
            return rest.trim_start().to_string();
        }
        if let Some(rest) = text.strip_prefix("--") {
            return rest.trim_start().to_string();
        }
    } else if kind.contains("block") {
        // Block comments: /* */, {- -}, etc.
        if let Some(rest) = text.strip_prefix("/*") {
            return rest.strip_suffix("*/").unwrap_or(rest).trim().to_string();
        }
        if let Some(rest) = text.strip_prefix("{-") {
            return rest.strip_suffix("-}").unwrap_or(rest).trim().to_string();
        }
    }

    // If no marker found, return as-is
    text.to_string()
}

/// Dummy extractor when tree-sitter is disabled.
#[cfg(not(feature = "tree-sitter"))]
pub struct TreeSitterExtractor;

#[cfg(not(feature = "tree-sitter"))]
pub fn get_extractor(_lang: Language) -> Option<TreeSitterExtractor> {
    None
}

#[cfg(all(test, feature = "tree-sitter"))]
mod tests {
    use super::*;
    use crate::config::{Pattern, PatternCategory, RegexPattern, Severity};

    #[test]
    fn test_python_extractor() {
        let mut extractor = get_extractor(Language::Python).expect("Python extractor");
        let code = r#"
# This is a comment
def foo():
    # TODO: implement this
    pass

# Another comment
"#;
        let comments = extractor.extract(code);
        assert!(comments.len() >= 2);
    }

    #[test]
    fn test_javascript_extractor() {
        let mut extractor = get_extractor(Language::JavaScript).expect("JS extractor");
        let code = r#"
// This is a comment
function foo() {
    // TODO: implement this
    return null;
}

// Another comment
"#;
        let comments = extractor.extract(code);
        assert!(comments.len() >= 2);
    }

    #[test]
    fn test_rust_extractor() {
        let mut extractor = get_extractor(Language::Rust).expect("Rust extractor");
        let code = r#"
// This is a comment
fn foo() -> Option<()> {
    // TODO: implement this
    None
}

// Another comment
"#;
        let comments = extractor.extract(code);
        assert!(comments.len() >= 2);
    }

    #[test]
    fn test_unsupported_language_unknown() {
        let extractor = get_extractor(Language::Unknown);
        assert!(extractor.is_none());
    }

    #[test]
    fn test_ast_query_not_implemented() {
        let mut extractor = get_extractor(Language::Python).expect("Python extractor");

        let patterns = vec![Pattern {
            regex: RegexPattern::new("raise NotImplementedError".to_string()).unwrap(),
            severity: Severity::Critical,
            message: "NotImplementedError stub detected".to_string(),
            category: PatternCategory::Stub,
            ast_query: Some("(raise_statement) @stub".to_string()),
            languages: vec!["Python".to_string()],
        }];

        let code = r#"
def process_data(data):
    raise NotImplementedError
"#;

        let findings = extractor.extract_ast_findings(code, &patterns);
        assert!(!findings.is_empty());
        assert_eq!(findings[0].severity, Severity::Critical);
    }

    #[test]
    fn test_ast_query_pass() {
        let mut extractor = get_extractor(Language::Python).expect("Python extractor");

        let patterns = vec![Pattern {
            regex: RegexPattern::new("pass$".to_string()).unwrap(),
            severity: Severity::Medium,
            message: "Function body contains only 'pass' statement".to_string(),
            category: PatternCategory::Stub,
            ast_query: Some("(pass_statement) @stub".to_string()),
            languages: vec!["Python".to_string()],
        }];

        let code = r#"
def stub_function():
    pass
"#;

        let findings = extractor.extract_ast_findings(code, &patterns);
        assert!(!findings.is_empty());
        assert_eq!(findings[0].severity, Severity::Medium);
    }

    #[test]
    fn test_ast_query_todo_macro() {
        let mut extractor = get_extractor(Language::Rust).expect("Rust extractor");

        let patterns = vec![Pattern {
            regex: RegexPattern::new("todo!".to_string()).unwrap(),
            severity: Severity::Critical,
            message: "todo!() macro stub detected".to_string(),
            category: PatternCategory::Stub,
            ast_query: Some("(macro_invocation) @stub".to_string()),
            languages: vec!["Rust".to_string()],
        }];

        let code = r#"
fn process_data() -> u32 {
    todo!()
}
"#;

        let findings = extractor.extract_ast_findings(code, &patterns);
        assert!(!findings.is_empty());
    }
}
