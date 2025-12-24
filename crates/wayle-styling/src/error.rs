//! Error types for styling compilation.

/// Errors that can occur during style compilation.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// SCSS compilation failed.
    #[error("SCSS compilation failed: {0}")]
    Compilation(#[from] Box<grass::Error>),
}
