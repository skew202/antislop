//! Profile caching for remote URLs.

use crate::{Error, Result};
use std::time::Duration;

/// Default TTL for cached profiles (24 hours).
pub const DEFAULT_CACHE_TTL: Duration = Duration::from_secs(24 * 60 * 60);

/// Fetch content from a URL.
///
/// This function uses a minimal HTTP client to fetch remote profiles.
/// It follows redirects and has a reasonable timeout.
pub fn fetch_url(url: &str) -> Result<String> {
    #[cfg(feature = "ureq")]
    {
        let client = ureq::AgentBuilder::new()
            .timeout(Duration::from_secs(30))
            .build();

        let response = client.get(url).call().map_err(|e| {
            Error::ConfigInvalid(format!("Failed to fetch profile from '{}': {}", url, e))
        })?;

        let status = response.status();
        if !(200..300).contains(&status) {
            return Err(Error::ConfigInvalid(format!(
                "Failed to fetch profile from '{}': HTTP {}",
                url, status
            )));
        }

        response.into_string().map_err(|e| {
            Error::ConfigInvalid(format!("Failed to read response from '{}': {}", url, e))
        })
    }

    #[cfg(not(feature = "ureq"))]
    {
        let _url = url; // Suppress unused warning
                        // Without ureq, provide a helpful error message
        Err(Error::ConfigInvalid(
            "Remote profile fetching requires the 'ureq' feature or 'remote-profiles' feature. \
            Enable it with: cargo build --features remote-profiles\n\
            Or download the profile manually and use --profile <path>"
                .to_string(),
        ))
    }
}

/// Check if a cached profile is still fresh based on its modification time.
pub fn is_cache_fresh(file_path: &std::path::Path, ttl: Duration) -> bool {
    let metadata = match std::fs::metadata(file_path) {
        Ok(m) => m,
        Err(_) => return false,
    };

    let modified = match metadata.modified() {
        Ok(m) => m,
        Err(_) => return false,
    };

    let elapsed = match std::time::SystemTime::now().duration_since(modified) {
        Ok(e) => e,
        Err(_) => return false,
    };

    elapsed < ttl
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_cache_fresh_missing_file() {
        assert!(!is_cache_fresh(
            std::path::Path::new("/nonexistent/file.toml"),
            DEFAULT_CACHE_TTL
        ));
    }

    #[cfg(feature = "ureq")]
    #[test]
    fn test_fetch_url_invalid() {
        let result = fetch_url("https://this-url-does-not-exist-12345.com");
        assert!(result.is_err());
    }
}
