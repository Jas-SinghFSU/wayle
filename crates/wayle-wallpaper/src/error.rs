//! Error types for the wallpaper service.

use std::{io, path::PathBuf};

/// Errors that can occur in the wallpaper service.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Service initialization failed.
    #[error("cannot initialize wallpaper service: {0}")]
    ServiceInitializationFailed(String),

    /// Wallpaper directory does not exist.
    #[error("directory not found: {}", .0.display())]
    DirectoryNotFound(PathBuf),

    /// No valid image files in the specified directory.
    #[error("no images found in directory: {}", .0.display())]
    NoImagesFound(PathBuf),

    /// Color extraction command failed to execute.
    #[error("cannot execute color extractor `{tool}`")]
    ColorExtractionCommandFailed {
        /// The tool that failed to execute.
        tool: &'static str,
        /// The underlying I/O error.
        #[source]
        source: io::Error,
    },

    /// Color extraction tool returned non-zero exit status.
    #[error("color extractor `{tool}` failed: {stderr}")]
    ColorExtractionFailed {
        /// The tool that failed.
        tool: &'static str,
        /// The stderr output from the tool.
        stderr: String,
    },

    /// Image file does not exist.
    #[error("image not found: {}", .0.display())]
    ImageNotFound(PathBuf),

    /// Image path contains invalid UTF-8.
    #[error("image path contains invalid UTF-8: {}", .0.display())]
    InvalidImagePath(PathBuf),

    /// Neither awww nor swww is installed.
    #[error("neither awww nor swww found in PATH")]
    AwwwNotInstalled,

    /// Wallpaper daemon is not running.
    #[error("wallpaper daemon is not running - start awww-daemon or swww-daemon")]
    AwwwDaemonNotRunning,

    /// Wallpaper command failed.
    #[error("wallpaper command failed: {stderr}")]
    AwwwCommandFailed {
        /// The stderr output from awww/swww.
        stderr: String,
    },

    /// Configuration path error.
    #[error("cannot access {context}")]
    ConfigPathError {
        /// What path operation failed.
        context: &'static str,
        /// The underlying I/O error.
        #[source]
        source: io::Error,
    },

    /// I/O operation failed.
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
}
