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

const MATUGEN_MAX_SOURCE_COLOR: u8 = 3;

/// Bundled color extractor tool and its parameters.
#[derive(Debug, Clone, PartialEq)]
pub struct ColorExtractorConfig {
    /// Which extraction tool to use.
    pub tool: ColorExtractor,
    /// Matugen scheme CLI value (e.g. "scheme-tonal-spot").
    pub matugen_scheme: String,
    /// Matugen contrast (-1.0 to 1.0).
    pub matugen_contrast: f64,
    /// Matugen source color index (0-3, clamped).
    pub matugen_source_color: u8,
    /// Matugen light mode.
    pub matugen_light: bool,
    /// Wallust palette config value (e.g. "dark16").
    pub wallust_palette: String,
    /// Wallust saturation boost (0-100, 0 = disabled).
    pub wallust_saturation: u8,
    /// Wallust contrast checking.
    pub wallust_check_contrast: bool,
    /// Wallust image sampling backend (e.g. "fastresize").
    pub wallust_backend: String,
    /// Wallust color space (e.g. "labmixed").
    pub wallust_colorspace: String,
    /// Pywal saturation (0.0-1.0).
    pub pywal_saturation: f64,
    /// Pywal contrast ratio (1.0-21.0).
    pub pywal_contrast: f64,
    /// Pywal light mode.
    pub pywal_light: bool,
    /// Apply wallust colors to terminals and external tools.
    pub wallust_apply_globally: bool,
    /// Apply pywal colors to terminals and external tools.
    pub pywal_apply_globally: bool,
}

impl Default for ColorExtractorConfig {
    fn default() -> Self {
        Self {
            tool: ColorExtractor::default(),
            matugen_scheme: "scheme-tonal-spot".into(),
            matugen_contrast: 0.0,
            matugen_source_color: 0,
            matugen_light: false,
            wallust_palette: "dark16".into(),
            wallust_saturation: 0,
            wallust_check_contrast: true,
            wallust_backend: "fastresize".into(),
            wallust_colorspace: "labmixed".into(),
            pywal_saturation: 0.05,
            pywal_contrast: 3.0,
            pywal_light: false,
            wallust_apply_globally: true,
            pywal_apply_globally: true,
        }
    }
}

impl ColorExtractorConfig {
    /// Extracts colors from an image using the configured tool and parameters.
    ///
    /// # Errors
    ///
    /// Returns error if the extraction command fails or the tool is not installed.
    #[instrument(skip(self), fields(extractor = %self.tool))]
    pub async fn extract(&self, image_path: &Path) -> Result<(), Error> {
        if self.tool == ColorExtractor::None {
            return Ok(());
        }

        let image_str = image_path.to_string_lossy();

        match self.tool {
            ColorExtractor::Wallust => {
                wallust::extract(
                    &image_str,
                    &self.wallust_palette,
                    self.wallust_saturation,
                    self.wallust_check_contrast,
                    &self.wallust_backend,
                    &self.wallust_colorspace,
                    self.wallust_apply_globally,
                )
                .await
            }
            ColorExtractor::Matugen => {
                let mode = if self.matugen_light { "light" } else { "dark" };
                matugen::extract(
                    &image_str,
                    &self.matugen_scheme,
                    self.matugen_contrast,
                    self.matugen_source_color.min(MATUGEN_MAX_SOURCE_COLOR),
                    mode,
                )
                .await
            }
            ColorExtractor::Pywal => {
                pywal::extract(
                    &image_str,
                    self.pywal_saturation,
                    self.pywal_contrast,
                    self.pywal_light,
                    self.pywal_apply_globally,
                )
                .await
            }
            ColorExtractor::None => Ok(()),
        }
    }
}

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
}

impl Tool {
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Pywal => "pywal",
            Self::Matugen => "matugen",
            Self::Wallust => "wallust",
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
