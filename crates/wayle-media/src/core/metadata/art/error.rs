use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ArtResolverError {
    #[error("cannot create cache directory")]
    CacheDir(#[source] std::io::Error),

    #[error("HTTP {status}")]
    HttpStatus { status: u16 },

    #[error("cannot download art")]
    Download(#[source] reqwest::Error),

    #[error("cannot write cache file")]
    WriteCache(#[source] std::io::Error),
}
