use std::error::Error as StdError;

use thiserror::Error;

pub(crate) fn error_chain(e: &dyn StdError) -> String {
    let mut msg = e.to_string();
    let mut source = e.source();
    while let Some(s) = source {
        msg.push_str(": ");
        msg.push_str(&s.to_string());
        source = s.source();
    }
    msg
}

/// Errors that can occur during weather operations.
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP request to weather provider failed.
    #[error("HTTP request to {provider} failed: {source}")]
    Http {
        /// Name of the weather provider that failed.
        provider: &'static str,
        /// The underlying HTTP error.
        #[source]
        source: reqwest::Error,
    },

    /// Weather provider returned an error status.
    #[error("{provider} returned HTTP {status}")]
    ProviderStatus {
        /// Name of the weather provider.
        provider: &'static str,
        /// HTTP status code returned.
        status: reqwest::StatusCode,
    },

    /// Cannot parse weather provider response.
    #[error("cannot parse {provider} response: {reason}")]
    Parse {
        /// Name of the weather provider.
        provider: &'static str,
        /// Description of the parse failure.
        reason: String,
    },

    /// Location not found by geocoding service.
    #[error("location '{query}' not found")]
    LocationNotFound {
        /// The location query that was not found.
        query: String,
    },

    /// Invalid location query format.
    #[error("invalid location format: {reason}")]
    InvalidLocation {
        /// Description of the format error.
        reason: String,
    },

    /// API key required but not configured.
    #[error("{provider} requires an API key in config")]
    ApiKeyMissing {
        /// Name of the provider requiring an API key.
        provider: &'static str,
    },

    /// API rate limit exceeded.
    #[error("{provider} rate limit exceeded")]
    RateLimited {
        /// Name of the rate-limited provider.
        provider: &'static str,
    },

    /// Weather data not available.
    #[error("weather data not yet available")]
    NotAvailable,
}

impl Error {
    pub(crate) fn http(provider: &'static str, source: reqwest::Error) -> Self {
        Self::Http { provider, source }
    }

    pub(crate) fn status(provider: &'static str, status: reqwest::StatusCode) -> Self {
        Self::ProviderStatus { provider, status }
    }

    pub(crate) fn parse(provider: &'static str, reason: impl Into<String>) -> Self {
        Self::Parse {
            provider,
            reason: reason.into(),
        }
    }

    pub(crate) fn is_retryable(&self) -> bool {
        match self {
            Self::Http { source, .. } => {
                source.is_timeout() || source.is_connect() || source.is_request()
            }
            Self::ProviderStatus { status, .. } => status.is_server_error(),
            Self::RateLimited { .. } => true,
            Self::Parse { .. }
            | Self::LocationNotFound { .. }
            | Self::InvalidLocation { .. }
            | Self::ApiKeyMissing { .. }
            | Self::NotAvailable => false,
        }
    }
}

/// Result type alias for weather operations.
pub type Result<T> = std::result::Result<T, Error>;
