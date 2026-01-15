//! swww backend for wallpaper rendering.

mod transition;

use std::{
    io::ErrorKind,
    path::Path,
    process::{Command, Stdio},
};

use tracing::instrument;
pub use transition::{
    BezierCurve, Position, TransitionAngle, TransitionConfig, TransitionDuration, TransitionFps,
    TransitionStep, TransitionType, WaveDimensions,
};

use crate::{Error, types::FitMode};

/// Backend for rendering wallpapers via swww-daemon.
#[derive(Debug, Clone, Copy, Default)]
pub struct SwwwBackend;

impl SwwwBackend {
    const RESIZE_FLAG: &'static str = "--resize";
    const OUTPUTS_FLAG: &'static str = "--outputs";

    /// Applies a wallpaper image to the specified monitor.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the image file
    /// * `fit_mode` - How to scale the image to fit the display
    /// * `monitor` - Monitor connector name (e.g., "DP-1"), or `None` for all monitors
    /// * `transition` - Transition animation configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the swww command fails or the daemon is not running.
    #[instrument(skip(transition), fields(path = %path.display(), monitor))]
    pub async fn apply(
        path: &Path,
        fit_mode: FitMode,
        monitor: Option<&str>,
        transition: &TransitionConfig,
    ) -> Result<(), Error> {
        let resize_mode = match fit_mode {
            FitMode::Fill => "crop",
            FitMode::Fit => "fit",
            FitMode::Center => "no",
            FitMode::Stretch => "stretch",
        };

        let path_str = path
            .to_str()
            .ok_or_else(|| Error::InvalidImagePath(path.to_path_buf()))?;

        let mut cmd = Command::new("swww");
        cmd.arg("img");
        cmd.arg(path_str);
        cmd.args([Self::RESIZE_FLAG, resize_mode]);

        apply_transition_args(&mut cmd, transition);

        if let Some(monitor) = monitor {
            cmd.args([Self::OUTPUTS_FLAG, monitor]);
        }

        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::piped());

        let output = cmd.output().map_err(|err| {
            if err.kind() == ErrorKind::NotFound {
                Error::SwwwNotInstalled
            } else {
                Error::Io(err)
            }
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

            if stderr.contains("daemon") || stderr.contains("socket") {
                return Err(Error::SwwwDaemonNotRunning);
            }

            return Err(Error::SwwwCommandFailed { stderr });
        }

        Ok(())
    }
}

fn apply_transition_args(cmd: &mut Command, config: &TransitionConfig) {
    cmd.args([TransitionType::FLAG, config.transition_type.type_name()]);
    cmd.args([TransitionDuration::FLAG, &config.duration.to_string()]);
    cmd.args([TransitionFps::FLAG, &config.fps.to_string()]);

    if let Some(step) = config.step {
        cmd.args([TransitionStep::FLAG, &step.to_string()]);
    }

    for (flag, value) in config.transition_type.cli_args() {
        cmd.args([flag, &value]);
    }
}
