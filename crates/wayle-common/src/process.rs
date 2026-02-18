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

/// Runs a shell command if non-empty, logging failures.
pub fn run_if_set(cmd: &str) {
    if cmd.is_empty() {
        return;
    }

    if let Err(e) = spawn_shell_quiet(cmd) {
        tracing::error!(error = %e, cmd = %cmd, "cannot spawn command");
    }
}

/// Action to perform on a bar module click or scroll event.
///
/// Serializes to/from a string for TOML config compatibility:
/// - `""` -> `None`
/// - `"dropdown:audio"` -> `Dropdown("audio")`
/// - `"pavucontrol"` -> `Shell("pavucontrol")`
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ClickAction {
    /// Open a named dropdown panel.
    Dropdown(String),
    /// Execute a shell command.
    Shell(String),
    #[default]
    /// No action configured.
    None,
}

impl ClickAction {
    fn from_str(s: &str) -> Self {
        if s.is_empty() {
            return Self::None;
        }
        match s.strip_prefix("dropdown:") {
            Some(name) => Self::Dropdown(name.to_owned()),
            None => Self::Shell(s.to_owned()),
        }
    }
}

impl serde::Serialize for ClickAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Dropdown(name) => serializer.serialize_str(&format!("dropdown:{name}")),
            Self::Shell(cmd) => serializer.serialize_str(cmd),
            Self::None => serializer.serialize_str(""),
        }
    }
}

impl<'de> serde::Deserialize<'de> for ClickAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from_str(&s))
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for ClickAction {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("ClickAction")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        String::json_schema(generator)
    }
}
