use std::{
    hash::{Hash, Hasher},
    path::PathBuf,
};

use fnv::FnvHasher;

use super::ArtResolverError;

const FILE_SCHEME: &str = "file://";
const HTTP_SCHEME: &str = "http://";
const HTTPS_SCHEME: &str = "https://";

const CACHE_SUBDIR: &str = "wayle";
const CACHE_ART_SUBDIR: &str = "media-art";
const FALLBACK_CACHE_BASE: &str = "/tmp";
const DEFAULT_CACHE_DIR: &str = ".cache";

const HASH_HEX_WIDTH: usize = 16;

/// Resolves album art URLs to local file paths, downloading and caching HTTP URLs.
#[derive(Debug, Clone)]
pub(crate) struct ArtResolver {
    cache_dir: PathBuf,
}

#[derive(Debug)]
pub(crate) enum ResolveResult {
    Ready(String),
    NeedsDownload { url: String, dest: PathBuf },
    Unresolvable,
}

impl ArtResolver {
    pub async fn new() -> Result<Self, ArtResolverError> {
        let cache_dir = cache_base_dir();
        tokio::fs::create_dir_all(&cache_dir)
            .await
            .map_err(ArtResolverError::CacheDir)?;

        Ok(Self { cache_dir })
    }

    pub fn resolve(&self, url: &str) -> ResolveResult {
        if let Some(local_path) = url.strip_prefix(FILE_SCHEME) {
            return ResolveResult::Ready(local_path.to_string());
        }

        if url.starts_with(HTTP_SCHEME) || url.starts_with(HTTPS_SCHEME) {
            let cached_path = self.cache_path(url);
            if cached_path.exists() {
                return match cached_path.to_str() {
                    Some(path) => ResolveResult::Ready(path.to_string()),
                    None => ResolveResult::Unresolvable,
                };
            }
            return ResolveResult::NeedsDownload {
                url: url.to_string(),
                dest: cached_path,
            };
        }

        ResolveResult::Unresolvable
    }

    pub async fn download(url: &str, dest: &PathBuf) -> Result<String, ArtResolverError> {
        let response = reqwest::get(url)
            .await
            .map_err(ArtResolverError::Download)?;

        if !response.status().is_success() {
            return Err(ArtResolverError::HttpStatus {
                status: response.status().as_u16(),
            });
        }

        let bytes = response.bytes().await.map_err(ArtResolverError::Download)?;
        tokio::fs::write(dest, &bytes)
            .await
            .map_err(ArtResolverError::WriteCache)?;

        dest.to_str().map(String::from).ok_or_else(|| {
            ArtResolverError::WriteCache(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "non-UTF-8 cache path",
            ))
        })
    }

    fn cache_path(&self, url: &str) -> PathBuf {
        let mut hasher = FnvHasher::default();
        url.hash(&mut hasher);
        let hash = hasher.finish();
        self.cache_dir.join(format!("{hash:0HASH_HEX_WIDTH$x}"))
    }
}

fn cache_base_dir() -> PathBuf {
    std::env::var("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| String::from(FALLBACK_CACHE_BASE));
            PathBuf::from(home).join(DEFAULT_CACHE_DIR)
        })
        .join(CACHE_SUBDIR)
        .join(CACHE_ART_SUBDIR)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hash_url(url: &str) -> u64 {
        let mut hasher = FnvHasher::default();
        url.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn hash_is_deterministic() {
        let first = hash_url("https://i.scdn.co/image/abc123");
        let second = hash_url("https://i.scdn.co/image/abc123");

        assert_eq!(first, second);
    }

    #[test]
    fn hash_differs_for_different_inputs() {
        let hash_a = hash_url("https://example.com/art1.png");
        let hash_b = hash_url("https://example.com/art2.png");

        assert_ne!(hash_a, hash_b);
    }

    #[test]
    fn resolve_file_url_returns_ready() {
        let resolver = ArtResolver {
            cache_dir: PathBuf::from("/tmp/test-cache"),
        };

        let result = resolver.resolve("file:///home/user/art.png");

        assert!(matches!(result, ResolveResult::Ready(path) if path == "/home/user/art.png"));
    }

    #[test]
    fn resolve_unknown_scheme_returns_unresolvable() {
        let resolver = ArtResolver {
            cache_dir: PathBuf::from("/tmp/test-cache"),
        };

        let result = resolver.resolve("ftp://example.com/art.png");

        assert!(matches!(result, ResolveResult::Unresolvable));
    }

    #[test]
    fn resolve_http_url_without_cache_returns_needs_download() {
        let resolver = ArtResolver {
            cache_dir: PathBuf::from("/tmp/nonexistent-cache-dir-xyz"),
        };

        let result = resolver.resolve("https://i.scdn.co/image/abc123");

        assert!(
            matches!(result, ResolveResult::NeedsDownload { url, .. } if url == "https://i.scdn.co/image/abc123")
        );
    }

    #[test]
    fn cache_path_uses_hex_filename_under_cache_dir() {
        let resolver = ArtResolver {
            cache_dir: PathBuf::from("/cache"),
        };

        let path = resolver.cache_path("https://example.com/art.png");
        let filename = path.file_name().unwrap().to_str().unwrap();

        assert_eq!(filename.len(), HASH_HEX_WIDTH);
        assert!(filename.chars().all(|ch| ch.is_ascii_hexdigit()));
        assert_eq!(path.parent().unwrap(), PathBuf::from("/cache"));
    }
}
