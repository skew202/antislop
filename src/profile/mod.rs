//! Community profiles for antislop.
//!
//! Profiles are shareable collections of slop detection patterns that can be
//! loaded from local files or remote URLs. They enable teams to define and
//! share coding standards without modifying antislop's core patterns.

pub mod cache;
pub mod validate;

use crate::config::{Pattern, PatternCategory};
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Profile metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileMetadata {
    /// Profile identifier.
    pub name: String,
    /// Semantic version.
    #[serde(default)]
    pub version: String,
    /// Human-readable description.
    #[serde(default)]
    pub description: String,
    /// Author or organization.
    #[serde(default)]
    pub author: String,
    /// URL to the profile repository.
    #[serde(default)]
    pub url: Option<String>,
    /// Minimum antislop version required.
    #[serde(default)]
    pub requires_version: Option<String>,
    /// Profiles this profile extends (inherits patterns from).
    #[serde(default)]
    pub extends: Vec<String>,
}

impl Default for ProfileMetadata {
    fn default() -> Self {
        Self {
            name: String::new(),
            version: "0.1.0".to_string(),
            description: String::new(),
            author: String::new(),
            url: None,
            requires_version: None,
            extends: Vec::new(),
        }
    }
}

/// A community profile containing slop detection patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// Profile metadata.
    #[serde(default)]
    pub metadata: ProfileMetadata,
    /// Detection patterns.
    #[serde(default)]
    pub patterns: Vec<Pattern>,
}

impl Profile {
    /// Create a new empty profile.
    pub fn new(name: String) -> Self {
        Self {
            metadata: ProfileMetadata {
                name,
                ..Default::default()
            },
            patterns: Vec::new(),
        }
    }

    /// Load a profile from a TOML file.
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path).map_err(|e| {
            Error::ConfigInvalid(format!(
                "Failed to read profile file '{}': {}",
                path.display(),
                e
            ))
        })?;

        Self::from_toml(&content)
    }

    /// Load a profile from a TOML string.
    pub fn from_toml(content: &str) -> Result<Self> {
        let profile: Self = toml::from_str(content)
            .map_err(|e| Error::ConfigInvalid(format!("Failed to parse profile TOML: {}", e)))?;

        // Validate the profile
        validate::validate_profile(&profile)?;

        Ok(profile)
    }

    /// Save a profile to a TOML file.
    pub fn to_file(&self, path: &Path) -> Result<()> {
        let content = self.to_toml()?;
        fs::write(path, content).map_err(|e| {
            Error::ConfigInvalid(format!(
                "Failed to write profile file '{}': {}",
                path.display(),
                e
            ))
        })?;
        Ok(())
    }

    /// Convert the profile to TOML format.
    pub fn to_toml(&self) -> Result<String> {
        toml::to_string_pretty(self)
            .map_err(|e| Error::ConfigInvalid(format!("Failed to serialize profile: {}", e)))
    }

    /// Merge another profile's patterns into this one.
    ///
    /// Patterns from the other profile are added after this profile's patterns.
    /// If both profiles have a pattern with the same regex and category, the
    /// other profile's pattern takes precedence.
    pub fn merge_with(&mut self, other: &Profile) {
        // Track (regex, category) pairs to avoid exact duplicates
        let mut seen: HashSet<(String, PatternCategory)> = HashSet::new();

        // First, deduplicate our own patterns
        self.patterns = self
            .patterns
            .drain(..)
            .rev()
            .filter(|p| seen.insert((p.regex.to_string(), p.category.clone())))
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();

        // Then add patterns from the other profile
        for pattern in &other.patterns {
            if !seen.contains(&(pattern.regex.to_string(), pattern.category.clone())) {
                self.patterns.push(pattern.clone());
                seen.insert((pattern.regex.to_string(), pattern.category.clone()));
            }
        }
    }

    /// Get all patterns from this profile.
    pub fn patterns(&self) -> &[Pattern] {
        &self.patterns
    }

    /// Get patterns for a specific category.
    pub fn patterns_for_category(&self, category: &PatternCategory) -> Vec<&Pattern> {
        self.patterns
            .iter()
            .filter(|p| &p.category == category)
            .collect()
    }
}

/// Source for loading a profile.
#[derive(Debug, Clone)]
pub enum ProfileSource {
    /// Local file path.
    Local(PathBuf),
    /// Remote URL (https://).
    Remote(String),
    /// Built-in profile name.
    Builtin(String),
}

impl ProfileSource {
    /// Parse a profile source from a string.
    ///
    /// - If it starts with "http://" or "https://", it's a Remote source.
    /// - If it exists as a file, it's a Local source.
    /// - Otherwise, it's a Builtin source (name only).
    pub fn parse(input: &str) -> Result<Self> {
        if input.starts_with("https://") || input.starts_with("http://") {
            return Ok(ProfileSource::Remote(input.to_string()));
        }

        let path = PathBuf::from(input);
        if path.exists() {
            return Ok(ProfileSource::Local(path));
        }

        Ok(ProfileSource::Builtin(input.to_string()))
    }
}

/// Profile loader with support for multiple sources.
pub struct ProfileLoader {
    /// Directory for caching remote profiles.
    cache_dir: PathBuf,
    /// Local project profile directory.
    project_dir: PathBuf,
    /// User config profile directory.
    user_dir: PathBuf,
}

impl ProfileLoader {
    /// Create a new profile loader with default directories.
    pub fn new() -> Result<Self> {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from(".cache"))
            .join("antislop")
            .join("profiles");

        let project_dir = PathBuf::from(".antislop").join("profiles");
        let user_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from(".config"))
            .join("antislop")
            .join("profiles");

        // Create cache directory if it doesn't exist
        fs::create_dir_all(&cache_dir).ok();

        Ok(Self {
            cache_dir,
            project_dir,
            user_dir,
        })
    }

    /// Create a profile loader with custom directories.
    pub fn with_dirs(cache_dir: PathBuf, project_dir: PathBuf, user_dir: PathBuf) -> Self {
        Self {
            cache_dir,
            project_dir,
            user_dir,
        }
    }

    /// Load a profile from the given source.
    ///
    /// Resolution order:
    /// 1. Explicit URL (fetch and cache)
    /// 2. Project-local `.antislop/profiles/<name>.toml`
    /// 3. User config `~/.config/antislop/profiles/<name>.toml`
    /// 4. Cache `~/.cache/antislop/profiles/<name>.toml`
    ///
    /// If the profile has `extends` entries, those profiles are loaded
    /// recursively and their patterns are merged.
    pub fn load(&self, source: &ProfileSource) -> Result<Profile> {
        let mut visited = std::collections::HashSet::new();
        self.load_with_extends(source, &mut visited)
    }

    /// Load a profile with extends resolution (internal).
    fn load_with_extends(
        &self,
        source: &ProfileSource,
        visited: &mut std::collections::HashSet<String>,
    ) -> Result<Profile> {
        // Load the base profile
        let mut profile = match source {
            ProfileSource::Remote(url) => self.load_remote(url),
            ProfileSource::Local(path) => Profile::from_file(path),
            ProfileSource::Builtin(name) => self.load_builtin(name),
        }?;

        // Check for circular extends
        let profile_id = profile.metadata.name.clone();
        if visited.contains(&profile_id) {
            return Err(Error::ConfigInvalid(format!(
                "Circular extends detected: '{}'",
                profile_id
            )));
        }
        visited.insert(profile_id);

        // Resolve extends
        let extends = std::mem::take(&mut profile.metadata.extends);
        for extend_name in extends {
            // Parse and load the extended profile
            let extend_source = ProfileSource::parse(&extend_name)?;
            match self.load_with_extends(&extend_source, visited) {
                Ok(extended) => {
                    // Merge extended profile's patterns (base patterns take precedence)
                    profile.merge_with(&extended);
                }
                Err(e) => {
                    // Log warning but continue - extends are optional
                    tracing::warn!("Failed to load extended profile '{}': {}", extend_name, e);
                }
            }
        }

        Ok(profile)
    }

    /// Load a profile by name, searching in multiple locations.
    pub fn load_by_name(&self, name: &str) -> Result<Profile> {
        // Try project-local first
        let project_path = self.project_dir.join(format!("{}.toml", name));
        if project_path.exists() {
            return Profile::from_file(&project_path);
        }

        // Try user config directory
        let user_path = self.user_dir.join(format!("{}.toml", name));
        if user_path.exists() {
            return Profile::from_file(&user_path);
        }

        // Try cache
        let cache_path = self.cache_dir.join(format!("{}.toml", name));
        if cache_path.exists() {
            return Profile::from_file(&cache_path);
        }

        Err(Error::ConfigInvalid(format!(
            "Profile '{}' not found. Searched in: {}, {}, {}",
            name,
            project_path.display(),
            user_path.display(),
            cache_path.display()
        )))
    }

    /// Load a remote profile from a URL.
    fn load_remote(&self, url: &str) -> Result<Profile> {
        // Check cache first - but only if fresh
        let cache_path = self.cache_path_for_url(url);
        if cache_path.exists() && cache::is_cache_fresh(&cache_path, cache::DEFAULT_CACHE_TTL) {
            if let Ok(profile) = Profile::from_file(&cache_path) {
                return Ok(profile);
            }
        }

        // Fetch from URL (cache expired or not present)
        let content = cache::fetch_url(url)?;
        let profile = Profile::from_toml(&content)?;

        // Cache the profile
        profile.to_file(&cache_path)?;

        Ok(profile)
    }

    /// Load a built-in profile by name.
    fn load_builtin(&self, name: &str) -> Result<Profile> {
        // First try loading by name (searches project, user, cache dirs)
        self.load_by_name(name)
    }

    /// Get the cache path for a URL.
    fn cache_path_for_url(&self, url: &str) -> PathBuf {
        // Create a hash of the URL for the filename
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        url.hash(&mut hasher);
        let hash = hasher.finish();

        self.cache_dir
            .join(format!("{}-{:016x}.toml", sanitize_name(url), hash))
    }

    /// Update all cached profiles by re-fetching from their sources.
    pub fn update_cache(&self) -> Result<Vec<String>> {
        let mut updated = Vec::new();

        for entry in
            fs::read_dir(&self.cache_dir).unwrap_or_else(|_| std::fs::read_dir(".").unwrap())
        {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            let path = entry.path();
            if path.extension().is_none_or(|e| e != "toml") {
                continue;
            }

            // Try to load and re-validate the profile
            if let Ok(profile) = Profile::from_file(&path) {
                if let Some(url) = &profile.metadata.url {
                    match cache::fetch_url(url) {
                        Ok(_) => updated.push(profile.metadata.name.clone()),
                        Err(_) => continue,
                    }
                }
            }
        }

        Ok(updated)
    }

    /// List all available profiles (project-local, user, and cached).
    pub fn list_available(&self) -> Vec<ProfileInfo> {
        let mut profiles = Vec::new();

        // Collect from all directories
        for dir in &[&self.project_dir, &self.user_dir, &self.cache_dir] {
            if !dir.exists() {
                continue;
            }

            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().is_none_or(|e| e != "toml") {
                        continue;
                    }

                    if let Ok(profile) = Profile::from_file(&path) {
                        profiles.push(ProfileInfo {
                            name: profile.metadata.name.clone(),
                            description: profile.metadata.description.clone(),
                            version: profile.metadata.version.clone(),
                            source: dir.to_path_buf(),
                            path,
                        });
                    }
                }
            }
        }

        profiles
    }
}

impl Default for ProfileLoader {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            cache_dir: PathBuf::from(".cache/profiles"),
            project_dir: PathBuf::from(".antislop/profiles"),
            user_dir: PathBuf::from(".config/profiles"),
        })
    }
}

/// Information about an available profile.
#[derive(Debug, Clone)]
pub struct ProfileInfo {
    /// Profile name.
    pub name: String,
    /// Profile description.
    pub description: String,
    /// Profile version.
    pub version: String,
    /// Source directory.
    pub source: PathBuf,
    /// Full path to the profile file.
    pub path: PathBuf,
}

/// Sanitize a name for use in a filename.
fn sanitize_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{RegexPattern, Severity};

    #[test]
    fn test_profile_new() {
        let profile = Profile::new("test-profile".to_string());
        assert_eq!(profile.metadata.name, "test-profile");
        assert!(profile.patterns.is_empty());
    }

    #[test]
    fn test_profile_from_toml() {
        let toml = r#"
            [metadata]
            name = "test-profile"
            version = "1.0.0"
            description = "A test profile"
            author = "test"

            [[patterns]]
            regex = "(?i)TODO:"
            severity = "medium"
            message = "TODO found"
            category = "placeholder"
        "#;

        let profile = Profile::from_toml(toml).unwrap();
        assert_eq!(profile.metadata.name, "test-profile");
        assert_eq!(profile.metadata.version, "1.0.0");
        assert_eq!(profile.patterns.len(), 1);
        assert_eq!(profile.patterns[0].message, "TODO found");
    }

    #[test]
    fn test_profile_merge_with() {
        let mut base = Profile {
            metadata: ProfileMetadata {
                name: "base".to_string(),
                ..Default::default()
            },
            patterns: vec![Pattern {
                regex: RegexPattern::new("(?i)TODO:".to_string()).unwrap(),
                severity: Severity::Medium,
                message: "TODO".to_string(),
                category: PatternCategory::Placeholder,
                ast_query: None,
                languages: vec![],
            }],
        };

        let extension = Profile {
            metadata: ProfileMetadata {
                name: "extension".to_string(),
                ..Default::default()
            },
            patterns: vec![Pattern {
                regex: RegexPattern::new("(?i)FIXME:".to_string()).unwrap(),
                severity: Severity::High,
                message: "FIXME".to_string(),
                category: PatternCategory::Placeholder,
                ast_query: None,
                languages: vec![],
            }],
        };

        base.merge_with(&extension);
        assert_eq!(base.patterns.len(), 2);
    }

    #[test]
    fn test_profile_source_parse_local() {
        // Create a temp file for testing
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test-profile.toml");
        std::fs::write(&test_file, "# test").unwrap();

        let source = ProfileSource::parse(test_file.to_str().unwrap()).unwrap();
        match source {
            ProfileSource::Local(p) => assert_eq!(p, test_file),
            _ => panic!("Expected Local source"),
        }

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn test_profile_source_parse_remote() {
        let source = ProfileSource::parse("https://example.com/profile.toml").unwrap();
        match source {
            ProfileSource::Remote(url) => assert_eq!(url, "https://example.com/profile.toml"),
            _ => panic!("Expected Remote source"),
        }
    }

    #[test]
    fn test_profile_source_parse_builtin() {
        let source = ProfileSource::parse("my-profile").unwrap();
        match source {
            ProfileSource::Builtin(name) => assert_eq!(name, "my-profile"),
            _ => panic!("Expected Builtin source"),
        }
    }

    #[test]
    fn test_sanitize_name() {
        assert_eq!(sanitize_name("test-profile"), "test_profile");
        assert_eq!(sanitize_name("test/profile"), "test_profile");
        assert_eq!(sanitize_name("test profile"), "test_profile");
    }
}
