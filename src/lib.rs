//! # antislop
//!
//! A blazing-fast, multi-language linter for detecting AI-generated code slop.
//!
//! AntiSlop identifies lazy placeholders, hedging language, stubs, and deferrals
//! commonly produced by quantized or rushed LLMs.
//!
//! ## Example
//!
//! ```no_run
//! use antislop::{Config, Scanner};
//!
//! let config = Config::default();
//! let scanner = Scanner::new(config.patterns).unwrap();
//! let content = "def foo():\n    # TODO: implement\n    pass\n";
//! let result = scanner.scan_file("example.py", content);
//! # Ok::<(), Box<dyn std::error::Error>>(())
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
pub mod filename_checker;
pub mod hygiene;
pub mod profile;
pub mod report;
pub mod walker;

#[doc(inline)]
pub use config::{Config, Pattern, PatternCategory, Severity};

#[doc(inline)]
pub use detector::{Comment, FileScanResult, Finding, ScanSummary, Scanner};

#[doc(inline)]
pub use filename_checker::{FilenameCheckConfig, FilenameChecker};

#[doc(inline)]
pub use report::{Format, Reporter};

#[doc(inline)]
pub use walker::Walker;

#[doc(inline)]
pub use profile::{Profile, ProfileLoader, ProfileSource};

/// Result type for antislop operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for antislop.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Config parsing error.
    /// Config parsing error.
    #[error("Config error: {0}")]
    Config(#[from] toml::de::Error),

    /// Missing or invalid config.
    #[error("Configuration invalid: {0}")]
    ConfigInvalid(String),

    /// Regex compilation error.
    #[error("Invalid regex: {0}")]
    Regex(#[from] regex::Error),

    /// Tree-sitter parsing error.
    #[cfg(feature = "tree-sitter")]
    #[error("Parse error: {0}")]
    Parse(String),
}

/// Version information.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default configuration file names.
pub const CONFIG_FILES: &[&str] = &["antislop.toml", ".antislop.toml", ".antislop"];
