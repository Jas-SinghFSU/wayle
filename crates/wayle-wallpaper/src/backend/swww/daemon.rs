//! swww-daemon lifecycle management.

use std::{
    process::{Command, Stdio},
    thread,
};

use tracing::{debug, info, warn};

/// Spawns swww-daemon in the background if not already running.
///
/// Runs entirely in a background thread - does not block startup.
/// Wallpaper operations will fail gracefully if the daemon isn't ready.
#[allow(clippy::cognitive_complexity)]
pub fn spawn_daemon_if_needed() {
    thread::spawn(|| match is_daemon_running() {
        Ok(true) => {
            debug!("swww-daemon already running");
        }
        Ok(false) => {
            info!("Starting swww-daemon");
            if let Err(e) = spawn_daemon() {
                warn!(error = %e, "cannot start swww-daemon");
            }
        }
        Err(e) => {
            warn!(error = %e, "cannot check swww-daemon status");
        }
    });
}

fn is_daemon_running() -> Result<bool, std::io::Error> {
    let output = Command::new("swww")
        .arg("query")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    Ok(output.success())
}

fn spawn_daemon() -> Result<(), std::io::Error> {
    Command::new("swww-daemon")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    Ok(())
}
