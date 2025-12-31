//! Parallel file traversal with gitignore support.

use crate::Config;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

/// A file entry from walking the directory tree.
#[derive(Debug, Clone)]
pub struct FileEntry {
    /// Full path to the file.
    pub path: PathBuf,
    /// File extension with leading dot.
    pub extension: Option<String>,
}

/// Parallel file walker.
pub struct Walker {
    /// File extensions to scan.
    extensions: Vec<String>,
    /// Maximum file size in bytes.
    max_file_size: u64,
}

impl Walker {
    /// Create a new walker.
    pub fn new(config: &Config) -> Self {
        Self {
            extensions: config.file_extensions.clone(),
            max_file_size: config.max_file_size_kb * 1024,
        }
    }

    /// Walk a directory and return matching files.
    pub fn walk(&self, paths: &[PathBuf]) -> Vec<FileEntry> {
        let mut entries = Vec::new();

        for base in paths {
            if !base.exists() {
                continue;
            }

            if base.is_file() {
                if self.matches_extension(base) {
                    entries.push(FileEntry {
                        path: base.clone(),
                        extension: Self::get_extension(base),
                    });
                }
                continue;
            }

            for entry in WalkBuilder::new(base)
                .standard_filters(true)
                .git_ignore(true)
                .git_exclude(true)
                .hidden(false)
                .max_filesize(Some(self.max_file_size))
                .build()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();

                if !path.is_file() {
                    continue;
                }

                if self.matches_extension(path) {
                    entries.push(FileEntry {
                        path: path.to_path_buf(),
                        extension: Self::get_extension(path),
                    });
                }
            }
        }

        entries
    }

    /// Check if a path matches the configured extensions.
    fn matches_extension(&self, path: &Path) -> bool {
        if self.extensions.contains(&"*".to_string()) {
            return true;
        }

        if let Some(ext) = Self::get_extension(path) {
            self.extensions.contains(&ext)
        } else {
            false
        }
    }

    /// Get the extension with leading dot.
    fn get_extension(path: &Path) -> Option<String> {
        path.extension()
            .and_then(|e| e.to_str())
            .map(|e| format!(".{}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_walker_filters_by_extension() {
        let temp = TempDir::new().unwrap();
        let dir = temp.path();

        File::create(dir.join("test.rs")).unwrap().write_all(b"fn main() {}").unwrap();
        File::create(dir.join("test.py")).unwrap().write_all(b"print('hi')").unwrap();
        File::create(dir.join("test.txt")).unwrap().write_all(b"ignore me").unwrap();

        let mut config = Config::default();
        config.file_extensions = vec![".rs".to_string()];

        let walker = Walker::new(&config);
        let files = walker.walk(&[dir.to_path_buf()]);

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].extension.as_deref(), Some(".rs"));
    }

    #[test]
    fn test_single_file() {
        let temp = TempDir::new().unwrap();
        let file = temp.path().join("test.rs");
        File::create(&file).unwrap().write_all(b"fn main() {}").unwrap();

        let config = Config::default();
        let walker = Walker::new(&config);
        let files = walker.walk(&[file.clone()]);

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, file);
    }
}
