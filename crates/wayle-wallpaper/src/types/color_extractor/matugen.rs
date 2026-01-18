//! Matugen color extraction (Material You colors).

use std::{fs, path::Path, process::Command};

use tracing::{debug, warn};
use wayle_config::ConfigPaths;

use super::Tool;
use crate::error::Error;

/// Matugen CLI arguments.
#[derive(Debug)]
pub enum Arg<'a> {
    /// `image <path>` - Extract colors from the specified image.
    Image(&'a Path),
    /// `--json <format>` - Output JSON in specified format.
    Json(&'static str),
}

impl Arg<'_> {
    fn apply(&self, cmd: &mut Command) {
        match self {
            Self::Image(path) => {
                cmd.args(["image", &path.to_string_lossy()]);
            }
            Self::Json(format) => {
                cmd.args(["--json", format]);
            }
        }
    }
}

fn run(args: &[Arg<'_>]) -> Result<Vec<u8>, Error> {
    let mut cmd = Command::new("matugen");

    for arg in args {
        arg.apply(&mut cmd);
    }

    let output = Tool::Matugen.run(cmd)?;
    Tool::Matugen.check_success(&output)?;
    Ok(output.stdout)
}

/// Runs matugen color extraction on the given image.
///
/// Saves JSON output to wayle's cache for wayle-styling to consume.
///
/// # Errors
///
/// Returns error if matugen command fails.
pub fn extract(image_path: &str) -> Result<(), Error> {
    let path = Path::new(image_path);
    let stdout = run(&[Arg::Image(path), Arg::Json("hex")])?;
    save_output(&stdout);
    Ok(())
}

fn save_output(stdout: &[u8]) {
    let cache_path = match ConfigPaths::matugen_colors() {
        Ok(path) => path,
        Err(err) => {
            warn!(error = %err, "cannot get matugen cache path");
            return;
        }
    };

    if let Err(err) = fs::write(&cache_path, stdout) {
        warn!(error = %err, path = %cache_path.display(), "cannot save matugen colors");
    } else {
        debug!(path = %cache_path.display(), "Saved matugen colors");
    }
}
