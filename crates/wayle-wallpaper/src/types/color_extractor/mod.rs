//! Color extraction from wallpaper images.

mod matugen;
mod pywal;
mod wallust;

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    path::Path,
    process::Output,
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use tokio::process::Command;
use tracing::instrument;

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
    /// Extracts colors from an image using the configured tool.
    ///
    /// For wallust and matugen, saves JSON output to wayle's cache directory
    /// for wayle-styling to consume.
    ///
    /// # Errors
    ///
    /// Returns error if the extraction command fails or the tool is not installed.
    #[instrument(skip(self), fields(extractor = %self))]
    pub async fn extract(self, image_path: &Path) -> Result<(), Error> {
        if self == Self::None {
            return Ok(());
        }

        let image_str = image_path.to_string_lossy();

        match self {
            Self::Wallust => wallust::extract(&image_str).await,
            Self::Matugen => matugen::extract(&image_str).await,
            Self::Pywal => pywal::extract(&image_str).await,
            Self::None => Ok(()),
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
        f.write_str(s)
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

/// Tool identifier for error messages and command building.
#[derive(Debug, Clone, Copy)]
pub(super) enum Tool {
    Pywal,
    Matugen,
    Wallust,
    WallustTemplates,
}

impl Tool {
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Pywal => "pywal",
            Self::Matugen => "matugen",
            Self::Wallust => "wallust",
            Self::WallustTemplates => "wallust (wayle templates)",
        }
    }

    pub(super) async fn run(self, mut cmd: Command) -> Result<Output, Error> {
        cmd.output()
            .await
            .map_err(|source| Error::ColorExtractionCommandFailed {
                tool: self.name(),
                source,
            })
    }

    pub(super) fn check_success(self, output: &Output) -> Result<(), Error> {
        if output.status.success() {
            return Ok(());
        }

        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(Error::ColorExtractionFailed {
            tool: self.name(),
            stderr,
        })
    }
}
