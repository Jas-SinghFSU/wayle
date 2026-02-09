use std::{process::Stdio, time::Duration};

use relm4::ComponentSender;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    time::{interval, timeout},
};
use tokio_stream::wrappers::IntervalStream;
use tokio_util::sync::CancellationToken;
use tracing::{debug, warn};
use wayle_common::{ConfigProperty, watch, watch_cancellable};
use wayle_config::schemas::modules::CustomModuleDefinition;

use super::{CustomModule, helpers, messages::CustomCmd};

const COMMAND_TIMEOUT: Duration = Duration::from_secs(30);
const SCROLL_DEBOUNCE: Duration = Duration::from_millis(50);

pub(super) fn spawn_command_poller(
    sender: &ComponentSender<CustomModule>,
    definition: &CustomModuleDefinition,
    token: CancellationToken,
) {
    if definition.command.is_none() {
        return;
    }

    let interval_ms = definition.interval_ms;
    let tick = interval(Duration::from_millis(interval_ms));
    let interval_stream = IntervalStream::new(tick);

    watch_cancellable!(sender, token, [interval_stream], |out| {
        let _ = out.send(CustomCmd::PollTrigger);
    });
}

pub(super) fn spawn_command_watcher(
    sender: &ComponentSender<CustomModule>,
    definition: &CustomModuleDefinition,
    token: CancellationToken,
) {
    let Some(command) = definition.command.clone() else {
        return;
    };

    let module_id = definition.id.clone();
    sender.command(move |out, shutdown| async move {
        let child_result = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true)
            .spawn();

        let mut child = match child_result {
            Ok(child) => child,
            Err(error) => {
                warn!(module_id = %module_id, error = %error, "failed to spawn watch command");
                return;
            }
        };

        debug!(module_id = %module_id, "watch command started");

        let Some(stdout) = child.stdout.take() else {
            return;
        };

        let mut reader = BufReader::new(stdout).lines();

        tokio::select! {
            () = shutdown.wait() => {
                debug!(module_id = %module_id, "watch command stopped (shutdown)");
            }
            () = token.cancelled() => {
                debug!(module_id = %module_id, "watch command stopped (config changed)");
            }
            () = async {
                while let Ok(Some(line)) = reader.next_line().await {
                    let _ = out.send(CustomCmd::WatchOutput(line));
                }
            } => {
                debug!(module_id = %module_id, "watch command exited");
            }
        }

        let _ = child.kill().await;
        let _ = child.wait().await;
    });
}

pub(super) fn spawn_config_watcher(
    sender: &ComponentSender<CustomModule>,
    custom_modules: &ConfigProperty<Vec<CustomModuleDefinition>>,
    module_id: String,
) {
    let custom_modules = custom_modules.clone();

    watch!(sender, [custom_modules.watch()], |out| {
        if let Some(definition) = helpers::find_definition(&custom_modules.get(), &module_id) {
            let _ = out.send(CustomCmd::DefinitionChanged(Box::new(definition)));
        } else {
            let _ = out.send(CustomCmd::DefinitionRemoved);
        }
    });
}

/// Spawns a debounced scroll action that fires after a quiet period.
///
/// If `cancel_token` is triggered before the debounce period expires, the
/// action is cancelled. Reset the token before each scroll to coalesce
/// rapid scrolls into a single on_action execution.
pub(super) fn spawn_scroll_debounce(
    sender: &ComponentSender<CustomModule>,
    cancel_token: CancellationToken,
) {
    sender.oneshot_command(async move {
        tokio::select! {
            biased;
            () = cancel_token.cancelled() => CustomCmd::CommandCancelled,
            () = tokio::time::sleep(SCROLL_DEBOUNCE) => CustomCmd::ScrollDebounceExpired,
        }
    });
}

/// Spawns a click action as fire-and-forget (no output captured).
///
/// Used for commands like `pavucontrol` that are intentionally long-lived.
/// The process runs independently and is not awaited.
pub(super) fn spawn_action(command: &str) {
    let command = command.to_string();
    tokio::spawn(async move {
        if let Err(error) = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            warn!(error = %error, "failed to spawn action command");
        }
    });
}

/// Runs a command asynchronously with timeout and single-flight cancellation.
///
/// If `cancel_token` is provided, the command will be cancelled if the token is triggered.
/// This enables single-flight behavior: reset the token before calling to cancel any
/// in-flight command.
pub(super) fn run_command_async(
    sender: &ComponentSender<CustomModule>,
    module_id: &str,
    command: String,
    cancel_token: CancellationToken,
) {
    let module_id = module_id.to_string();
    sender.oneshot_command(async move {
        let outcome = tokio::select! {
            biased;
            () = cancel_token.cancelled() => ExecOutcome::Cancelled,
            result = timeout(COMMAND_TIMEOUT, run_command(&command)) => match result {
                Ok(Ok(output)) => ExecOutcome::Output(output),
                Ok(Err(error)) => ExecOutcome::Failed(error),
                Err(_) => ExecOutcome::TimedOut,
            },
        };

        map_exec_outcome(&module_id, outcome)
    });
}

enum ExecOutcome {
    Output(String),
    Cancelled,
    TimedOut,
    Failed(std::io::Error),
}

fn map_exec_outcome(module_id: &str, outcome: ExecOutcome) -> CustomCmd {
    match outcome {
        ExecOutcome::Output(output) => CustomCmd::CommandOutput(output),
        ExecOutcome::Cancelled => CustomCmd::CommandCancelled,
        ExecOutcome::TimedOut => {
            warn!(
                module_id = %module_id,
                timeout_secs = COMMAND_TIMEOUT.as_secs(),
                "command timed out"
            );
            CustomCmd::CommandCancelled
        }
        ExecOutcome::Failed(error) => {
            warn!(module_id = %module_id, error = %error, "command execution failed");
            CustomCmd::CommandCancelled
        }
    }
}

async fn run_command(command: &str) -> Result<String, std::io::Error> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .output()
        .await?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_exec_outcome_output() {
        let cmd = map_exec_outcome("test", ExecOutcome::Output(String::from("ok")));
        assert!(matches!(cmd, CustomCmd::CommandOutput(output) if output == "ok"));
    }

    #[test]
    fn map_exec_outcome_cancelled() {
        let cmd = map_exec_outcome("test", ExecOutcome::Cancelled);
        assert!(matches!(cmd, CustomCmd::CommandCancelled));
    }

    #[test]
    fn map_exec_outcome_timeout() {
        let cmd = map_exec_outcome("test", ExecOutcome::TimedOut);
        assert!(matches!(cmd, CustomCmd::CommandCancelled));
    }

    #[test]
    fn map_exec_outcome_failed() {
        let error = std::io::Error::other("boom");
        let cmd = map_exec_outcome("test", ExecOutcome::Failed(error));
        assert!(matches!(cmd, CustomCmd::CommandCancelled));
    }
}
