use super::proxy::{connect, format_error};
use crate::cli::CliAction;

pub async fn execute() -> CliAction {
    let (_connection, proxy) = connect().await?;

    proxy
        .disable()
        .await
        .map_err(|e| format_error("disable idle inhibit", e))?;

    println!("Disabled");

    Ok(())
}
