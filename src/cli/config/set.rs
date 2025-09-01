use crate::{cli::CliAction, config_runtime::ConfigRuntime};

pub async fn execute(path: String, value: String) -> CliAction {
    let config_runtime =
        ConfigRuntime::load().map_err(|e| format!("Failed to load config: {e}"))?;

    let toml_value = parse_toml_value(&value)?;

    config_runtime
        .set_by_path(&path, toml_value)
        .map_err(|e| format!("Failed to set config at '{path}': {e}"))?;

    println!("Set {path} = {value}");

    Ok(())
}

fn parse_toml_value(value: &str) -> Result<toml::Value, String> {
    let toml_container = format!("value = {value}");

    match toml::from_str::<toml::Table>(&toml_container) {
        Ok(mut table) => table
            .remove("value")
            .ok_or_else(|| String::from("Failed to parse value")),
        Err(_) => Ok(toml::Value::String(value.to_string())),
    }
}
