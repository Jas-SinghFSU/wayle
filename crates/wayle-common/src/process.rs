//! Shell command execution utilities.

use std::{
    io,
    process::{Child as StdChild, Command as StdCommand, Stdio},
};

use tokio::process::Command;
use tracing::warn;

/// Spawns a shell command via `sh -c` (async).
///
/// # Errors
///
/// Returns error if the shell process cannot be spawned.
pub fn spawn_shell(cmd: &str) -> io::Result<tokio::process::Child> {
    Command::new("sh").arg("-c").arg(cmd).spawn()
}

/// Spawns a shell command, discarding stdout but logging failures.
///
/// A background task monitors the child and logs a warning if it exits
/// with non-zero status or produces stderr output.
///
/// # Errors
///
/// Returns error if the shell process cannot be spawned.
pub fn spawn_shell_quiet(cmd: &str) -> io::Result<()> {
    let child = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()?;

    let cmd = cmd.to_owned();
    tokio::spawn(async move {
        match child.wait_with_output().await {
            Ok(output) if !output.status.success() => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let stderr = stderr.trim();
                if stderr.is_empty() {
                    warn!(cmd = %cmd, exit_code = ?output.status.code(), "command failed");
                } else {
                    warn!(
                        cmd = %cmd,
                        exit_code = ?output.status.code(),
                        stderr = %stderr,
                        "command failed"
                    );
                }
            }
            Err(e) => {
                warn!(cmd = %cmd, error = %e, "cannot wait on command");
            }
            Ok(_) => {}
        }
    });

    Ok(())
}

/// Spawns a shell command via `sh -c` (sync).
///
/// # Errors
///
/// Returns error if the shell process cannot be spawned.
pub fn spawn_shell_sync(cmd: &str) -> io::Result<StdChild> {
    StdCommand::new("sh").arg("-c").arg(cmd).spawn()
}

/// Spawns a shell command via `sh -c` (sync), discarding output.
///
/// # Errors
///
/// Returns error if the shell process cannot be spawned.
pub fn spawn_shell_sync_quiet(cmd: &str) -> io::Result<StdChild> {
    StdCommand::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
}

/// Runs a config-defined shell command if non-empty.
///
/// Logs an error if spawning fails. Does nothing if `cmd` is empty.
pub fn run_if_set(cmd: &str) {
    if cmd.is_empty() {
        return;
    }

    if let Err(e) = spawn_shell_quiet(cmd) {
        tracing::error!(error = %e, cmd = %cmd, "failed to spawn command");
    }
}
