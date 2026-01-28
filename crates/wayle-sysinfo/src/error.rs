use thiserror::Error;

/// Errors that can occur in the sysinfo service.
#[derive(Debug, Error)]
pub enum Error {
    /// System information unavailable.
    #[error("system information unavailable: {0}")]
    Unavailable(String),
}
