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
            .adjust_remaining(delta)
            .await
            .map_err(|e| format_error("adjust remaining", e))?;

        println!("Added {delta} minutes to remaining");
    } else if value.starts_with('-') {
        let delta: i32 = value
            .parse()
            .map_err(|_| format!("Invalid delta: {value}"))?;

        proxy
            .adjust_remaining(delta)
            .await
            .map_err(|e| format_error("adjust remaining", e))?;

        println!("Subtracted {} minutes from remaining", delta.abs());
    } else {
        let minutes: u32 = value
            .parse()
            .map_err(|_| format!("Invalid minutes: {value}"))?;

        proxy
            .set_remaining(minutes)
            .await
            .map_err(|e| format_error("set remaining", e))?;

        println!("Set remaining to {minutes} minutes");
    }

    Ok(())
}
