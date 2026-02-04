use super::proxy::{connect, format_error};
use crate::cli::CliAction;

pub async fn execute(value: String) -> CliAction {
    let (_connection, proxy) = connect().await?;

    let value = value.trim();

    if let Some(stripped) = value.strip_prefix('+') {
        let delta: i32 = stripped
            .parse()
            .map_err(|_| format!("Invalid delta: {value}"))?;

        proxy
            .adjust_duration(delta)
            .await
            .map_err(|e| format_error("adjust duration", e))?;

        println!("Added {delta} minutes to duration");
    } else if value.starts_with('-') {
        let delta: i32 = value
            .parse()
            .map_err(|_| format!("Invalid delta: {value}"))?;

        proxy
            .adjust_duration(delta)
            .await
            .map_err(|e| format_error("adjust duration", e))?;

        println!("Subtracted {} minutes from duration", delta.abs());
    } else {
        let minutes: u32 = value
            .parse()
            .map_err(|_| format!("Invalid minutes: {value}"))?;

        proxy
            .set_duration(minutes)
            .await
            .map_err(|e| format_error("set duration", e))?;

        println!("Set duration to {minutes} minutes");
    }

    Ok(())
}
