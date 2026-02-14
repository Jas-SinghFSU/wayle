//! Wallpaper daemon lifecycle management.

use std::{
    io,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use tracing::{debug, info, warn};

const READY_POLL_INTERVAL: Duration = Duration::from_millis(50);
const READY_TIMEOUT: Duration = Duration::from_secs(5);

/// Spawns the wallpaper daemon in the background if not already running.
///
/// Prefers `awww-daemon`, falling back to `swww-daemon` if awww is not
/// installed. Polls until the daemon is accepting commands, then signals
/// readiness via [`super::DAEMON_READY`].
pub fn spawn_daemon_if_needed() {
    thread::spawn(|| {
        let daemon = super::daemon_binary();

        match is_daemon_running() {
            Ok(true) => {
                debug!("{daemon} already running");
                super::DAEMON_READY.notify_one();
                return;
            }
            Ok(false) => {
                info!("Starting {daemon}");
                if let Err(e) = start_daemon() {
                    warn!(error = %e, "cannot start {daemon}");
                    super::DAEMON_READY.notify_one();
                    return;
                }
            }
            Err(e) => {
                warn!(error = %e, "cannot check {daemon} status");
                super::DAEMON_READY.notify_one();
                return;
            }
        }

        wait_until_ready(daemon);
        super::DAEMON_READY.notify_one();
    });
}

fn wait_until_ready(daemon: &str) {
    let start = Instant::now();

    while start.elapsed() < READY_TIMEOUT {
        thread::sleep(READY_POLL_INTERVAL);

        match is_daemon_running() {
            Ok(true) => {
                debug!(elapsed_ms = start.elapsed().as_millis(), "{daemon} ready");
                return;
            }
            Ok(false) | Err(_) => {}
        }
    }

    warn!("{daemon} not ready after {}s", READY_TIMEOUT.as_secs());
}

fn is_daemon_running() -> Result<bool, io::Error> {
    let output = Command::new(super::client_binary())
        .arg("query")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    Ok(output.success())
}

fn start_daemon() -> Result<(), io::Error> {
    Command::new(super::daemon_binary())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    Ok(())
}
