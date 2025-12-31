//! # antislop
//!
//! A blazing-fast, multi-language linter for detecting AI-generated code slop.
//!
//! Antislop identifies lazy placeholders, hedging language, stubs, and deferrals
//! commonly produced by quantized or rushed LLMs.
//!
//! ## Example
//!
//! ```no_run
//! use antislop::{Config, Scanner};
//!
//! let config = Config::default();
//! let scanner = Scanner::new(config.patterns)?;
//! let result = scanner.scan_file("path/to/file.py", &content);
//! ```
//!
//! ## Slop Categories
//!
//! - **Placeholder**: TODO, FIXME, HACK, NOTE, XXX comments
//! - **Deferral**: "for now", "temporary", "quick implementation"
//! - **Hedging**: "hopefully", "should work", "this is a simple"
//! - **Stub**: Empty functions near placeholder comments

pub mod config;
pub mod detector;
pub mod report;
pub mod walker;

#[doc(inline)]
pub use config::{Config, Pattern, PatternCategory, Severity};

#[doc(inline)]
pub use detector::{Comment, Finding, FileScanResult, Scanner, ScanSummary};

#[doc(inline)]
pub use report::{Format, Reporter};

#[doc(inline)]
pub use walker::Walker;

/// Result type for antislop operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for antislop.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Config parsing error.
    #[error("Config error: {0}")]
    Config(String),

    /// Regex compilation error.
    #[error("Invalid regex '{0}': {1}")]
    Regex(String, String),

    /// Tree-sitter parsing error.
    #[cfg(feature = "tree-sitter")]
    #[error("Parse error: {0}")]
    Parse(String),
}

/// Version information.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default configuration file names.
pub const CONFIG_FILES: &[&str] = &["antislop.toml", ".antislop.toml", ".antislop"];
