//! Error types for the wallpaper service.

use std::{io, path::PathBuf};

/// Errors that can occur in the wallpaper service.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Service initialization failed.
    #[error("Failed to initialize wallpaper service: {0}")]
    ServiceInitializationFailed(String),

    /// Wallpaper directory does not exist.
    #[error("Directory not found: {}", .0.display())]
    DirectoryNotFound(PathBuf),

    /// No valid image files in the specified directory.
    #[error("No images found in directory: {}", .0.display())]
    NoImagesFound(PathBuf),

    /// Color extraction tool failed.
    #[error("Color extraction failed ({tool}): {reason}")]
    ColorExtractionFailed {
        /// The tool that failed.
        tool: String,
        /// The failure reason.
        reason: String,
    },

    /// Failed to load an image file.
    #[error("Failed to load image {}: {reason}", .path.display())]
    ImageLoadFailed {
        /// Path to the image that failed to load.
        path: PathBuf,
        /// The failure reason.
        reason: String,
    },

    /// swww is not installed or not in PATH.
    #[error("swww is not installed or not in PATH")]
    SwwwNotInstalled,

    /// swww-daemon is not running.
    #[error("swww-daemon is not running - start it with `swww-daemon`")]
    SwwwDaemonNotRunning,

    /// swww command failed.
    #[error("swww command failed: {reason}")]
    SwwwCommandFailed {
        /// The failure reason from stderr.
        reason: String,
    },

    /// I/O operation failed.
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
}
