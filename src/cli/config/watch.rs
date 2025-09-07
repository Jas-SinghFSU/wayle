use crate::{cli::CliAction, config_runtime::runtime::ConfigRuntime};

/// Execute the command
pub async fn execute(path: String) -> CliAction {
    let config_runtime =
        ConfigRuntime::load().map_err(|e| format!("Failed to load config: {e}"))?;

    println!("Watching changes on path '{path}'...");
    println!("Press Ctrl+C to stop");

    let _file_watch_handle = config_runtime
        .start_file_watching()
        .map_err(|e| format!("Failed to start file watching: {e}"))?;

    let mut subscription = config_runtime
        .subscribe_to_path(&path)
        .await
        .map_err(|e| format!("Failed to subscribe to path '{path}': {e}"))?;

    while let Some(change) = subscription.receiver_mut().recv().await {
        println!(
            "[{}s] {} -> {}",
            change.timestamp.elapsed().as_secs(),
            change.path,
            format_toml_value(&change.new_value)
        );
    }

    Ok(())
}

fn format_toml_value(value: &toml::Value) -> String {
    match value {
        toml::Value::String(s) => s.clone(),
        toml::Value::Integer(i) => i.to_string(),
        toml::Value::Float(f) => f.to_string(),
        toml::Value::Boolean(b) => b.to_string(),
        toml::Value::Datetime(d) => d.to_string(),
        toml::Value::Array(_) | toml::Value::Table(_) => {
            toml::to_string_pretty(value).unwrap_or_else(|_| value.to_string())
        }
    }
}
