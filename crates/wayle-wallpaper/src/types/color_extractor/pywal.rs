//! Pywal color extraction.

use std::path::Path;

use tokio::process::Command;

use super::Tool;
use crate::error::Error;

/// Pywal CLI arguments.
#[derive(Debug)]
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
    /// `--saturate <value>` - Set color saturation (0.0 to 1.0).
    Saturate(f64),
    /// `--contrast <value>` - Minimum contrast ratio (1.0 to 21.0).
    Contrast(f64),
    /// `-l` - Use light mode.
    Light,
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
            Self::Saturate(value) => {
                cmd.args(["--saturate", &value.to_string()]);
            }
            Self::Contrast(value) => {
                cmd.args(["--contrast", &value.to_string()]);
            }
            Self::Light => {
                cmd.arg("-l");
            }
        }
    }
}

async fn run(args: &[Arg<'_>]) -> Result<(), Error> {
    let mut cmd = Command::new("wal");

    for arg in args {
        arg.apply(&mut cmd);
    }

    let output = Tool::Pywal.run(cmd).await?;
    Tool::Pywal.check_success(&output)
}

/// Runs pywal color extraction on the given image.
///
/// # Errors
///
/// Returns error if pywal command fails.
pub async fn extract(
    image_path: &str,
    saturation: f64,
    contrast: f64,
    light: bool,
    apply_globally: bool,
) -> Result<(), Error> {
    let path = Path::new(image_path);
    let mut args = vec![
        Arg::Image(path),
        Arg::NoWallpaper,
        Arg::Saturate(saturation),
        Arg::Contrast(contrast),
    ];
    if light {
        args.push(Arg::Light);
    }
    if !apply_globally {
        args.push(Arg::SkipTerminal);
        args.push(Arg::SkipTty);
        args.push(Arg::SkipReload);
    }
    run(&args).await
}
