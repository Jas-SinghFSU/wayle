use super::proxy::{connect, format_error};
use crate::cli::CliAction;

pub async fn execute(minutes: Option<u32>, indefinite: bool) -> CliAction {
    let (_connection, proxy) = connect().await?;

    if let Some(mins) = minutes {
        proxy
            .set_duration(mins)
            .await
            .map_err(|e| format_error("set duration", e))?;
    }

    proxy
        .enable(indefinite)
        .await
        .map_err(|e| format_error("enable idle inhibit", e))?;

    let duration = proxy.duration().await.unwrap_or(0);
    if indefinite || duration == 0 {
        println!("Enabled (indefinite)");
    } else {
        println!("Enabled for {duration} minutes");
    }

    Ok(())
}
