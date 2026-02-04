use super::proxy::{connect, format_error};
use crate::cli::CliAction;

pub async fn execute(indefinite: bool) -> CliAction {
    let (_connection, proxy) = connect().await?;

    let active = proxy
        .active()
        .await
        .map_err(|e| format_error("get active state", e))?;

    if active {
        proxy
            .disable()
            .await
            .map_err(|e| format_error("disable idle inhibit", e))?;
        println!("Disabled");
        return Ok(());
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
