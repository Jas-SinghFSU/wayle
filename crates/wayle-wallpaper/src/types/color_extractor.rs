//! Color extraction via external tools.

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    fs,
    path::Path,
    process::Command,
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use tracing::{debug, instrument, warn};
use wayle_config::ConfigPaths;

use crate::error::Error;

/// External tool used for extracting colors from wallpaper images.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ColorExtractor {
    /// Use wallust for color extraction.
    #[default]
    Wallust,
    /// Use matugen for Material You colors.
    Matugen,
    /// Use pywal for color extraction.
    Pywal,
    /// Disable color extraction.
    None,
}

impl ColorExtractor {
    /// Returns the command name for this extractor.
    pub fn command(self) -> Option<&'static str> {
        match self {
            Self::Wallust => Some("wallust"),
            Self::Matugen => Some("matugen"),
            Self::Pywal => Some("wal"),
            Self::None => None,
        }
    }

    /// Extracts colors from an image using the configured tool.
    ///
    /// For matugen, also saves JSON output to wayle's cache directory for
    /// wayle-styling to consume.
    ///
    /// # Errors
    ///
    /// Returns error if the extraction command fails or the tool is not installed.
    #[instrument(skip(self), fields(extractor = %self))]
    pub async fn extract(self, image_path: &Path) -> Result<(), Error> {
        let Some(cmd) = self.command() else {
            return Ok(());
        };

        let image_str = image_path.to_string_lossy();

        let args: Vec<&str> = match self {
            Self::Wallust => vec!["run", &image_str],
            Self::Matugen => vec!["image", &image_str, "--json", "hex"],
            Self::Pywal => vec!["-i", &image_str, "-n"],
            Self::None => return Ok(()),
        };

        let output =
            Command::new(cmd)
                .args(&args)
                .output()
                .map_err(|e| Error::ColorExtractionFailed {
                    tool: cmd.to_string(),
                    reason: format!("Failed to execute: {e}"),
                })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::ColorExtractionFailed {
                tool: cmd.to_string(),
                reason: stderr.to_string(),
            });
        }

        if self == Self::Matugen {
            self.save_matugen_output(&output.stdout);
        }

        Ok(())
    }

    fn save_matugen_output(&self, stdout: &[u8]) {
        let cache_path = match ConfigPaths::matugen_colors() {
            Ok(path) => path,
            Err(err) => {
                warn!(error = %err, "Failed to get matugen cache path");
                return;
            }
        };

        if let Err(err) = fs::write(&cache_path, stdout) {
            warn!(error = %err, path = %cache_path.display(), "Failed to save matugen colors");
        } else {
            debug!(path = %cache_path.display(), "Saved matugen colors");
        }
    }
}

impl Display for ColorExtractor {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let s = match self {
            Self::Wallust => "wallust",
            Self::Matugen => "matugen",
            Self::Pywal => "pywal",
            Self::None => "none",
        };
        write!(f, "{s}")
    }
}

impl FromStr for ColorExtractor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "wallust" => Ok(Self::Wallust),
            "matugen" => Ok(Self::Matugen),
            "pywal" | "wal" => Ok(Self::Pywal),
            "none" | "disabled" => Ok(Self::None),
            _ => Err(format!("Invalid color extractor: {s}")),
        }
    }
}
