use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the set command.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute(profile: String) -> CliAction {
    let (_connection, proxy) = connect().await?;

    proxy
        .set_profile(profile.clone())
        .await
        .map_err(|e| format_error("set profile", e))?;

    println!("Profile set to: {profile}");

    Ok(())
}
