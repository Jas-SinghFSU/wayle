//! Pywal color extraction.

use std::{path::Path, process::Command};

use super::Tool;
use crate::error::Error;

/// Pywal CLI arguments.
#[derive(Debug)]
#[allow(dead_code)]
pub enum Arg<'a> {
    /// `-i <path>` - Image file to extract colors from.
    Image(&'a Path),
    /// `-n` - Skip setting the wallpaper.
    NoWallpaper,
    /// `-s` - Skip changing colors in terminals.
    SkipTerminal,
    /// `-t` - Skip changing colors in TTY.
    SkipTty,
    /// `-e` - Skip reloading gtk/xrdb/i3/sway/polybar.
    SkipReload,
}

impl Arg<'_> {
    fn apply(&self, cmd: &mut Command) {
        match self {
            Self::Image(path) => {
                cmd.args(["-i", &path.to_string_lossy()]);
            }
            Self::NoWallpaper => {
                cmd.arg("-n");
            }
            Self::SkipTerminal => {
                cmd.arg("-s");
            }
            Self::SkipTty => {
                cmd.arg("-t");
            }
            Self::SkipReload => {
                cmd.arg("-e");
            }
        }
    }
}

fn run(args: &[Arg<'_>]) -> Result<(), Error> {
    let mut cmd = Command::new("wal");

    for arg in args {
        arg.apply(&mut cmd);
    }

    let output = Tool::Pywal.run(cmd)?;
    Tool::Pywal.check_success(&output)
}

/// Runs pywal color extraction on the given image.
///
/// Pywal writes colors to its own cache location (`~/.cache/wal/colors.json`).
///
/// # Errors
///
/// Returns error if pywal command fails.
pub fn extract(image_path: &str) -> Result<(), Error> {
    let path = Path::new(image_path);
    run(&[Arg::Image(path), Arg::NoWallpaper])
}
