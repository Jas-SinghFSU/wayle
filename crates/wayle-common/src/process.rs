//! Shell command execution utilities.

use std::{
    io,
    process::{Child as StdChild, Command as StdCommand},
};

use tokio::process::{Child, Command};

/// Spawns a shell command via `sh -c` (async).
///
/// # Errors
///
/// Returns error if the shell process cannot be spawned.
pub fn spawn_shell(cmd: &str) -> io::Result<Child> {
    Command::new("sh").arg("-c").arg(cmd).spawn()
}

/// Spawns a shell command via `sh -c` (sync).
///
/// # Errors
///
/// Returns error if the shell process cannot be spawned.
pub fn spawn_shell_sync(cmd: &str) -> io::Result<StdChild> {
    StdCommand::new("sh").arg("-c").arg(cmd).spawn()
}
