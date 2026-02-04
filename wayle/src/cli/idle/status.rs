use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the status command.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute() -> CliAction {
    let (_connection, proxy) = connect().await?;

    let active = proxy
        .active()
        .await
        .map_err(|e| format_error("get active state", e))?;

    let duration = proxy
        .duration()
        .await
        .map_err(|e| format_error("get duration", e))?;

    if !active {
        let dur_str = if duration == 0 {
            "indefinite"
        } else {
            &format!("{duration} min")
        };
        println!("Inactive (duration: {dur_str})");
        return Ok(());
    }

    if duration == 0 {
        println!("Active (indefinite)");
        return Ok(());
    }

    let remaining = proxy
        .remaining()
        .await
        .map_err(|e| format_error("get remaining", e))?;

    let remaining_mins = remaining / 60;
    let remaining_secs = remaining % 60;

    println!("Active ({remaining_mins}:{remaining_secs:02} remaining, {duration} min duration)");

    Ok(())
}
