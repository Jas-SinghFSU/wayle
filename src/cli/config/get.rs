use crate::{cli::CliAction, config_runtime::ConfigRuntime};

pub async fn execute(path: String) -> CliAction {
    let config_runtime =
        ConfigRuntime::load().map_err(|e| format!("Failed to load config: {e}"))?;

    let value = config_runtime
        .get_by_path(&path)
        .map_err(|e| format!("Failed to get config at '{path}': {e}"))?;

    println!("{}", format_toml_value(&value));

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
