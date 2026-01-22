use std::fs;

use schemars::schema_for;
use wayle_config::{Config, ConfigPaths};

use crate::cli::CliAction;

const TAPLO_CONFIG: &str = r#"[schema]
path = "./schema.json"
"#;

/// Generates JSON Schema and taplo config for editor intellisense.
///
/// Writes `schema.json` and `.taplo.toml` to `~/.config/wayle/`.
/// Use `stdout` flag to print schema to terminal instead.
///
/// # Errors
///
/// Returns error if schema serialization or file write fails.
pub fn execute(stdout: bool) -> CliAction {
    let schema = schema_for!(Config);
    let json = serde_json::to_string_pretty(&schema)
        .map_err(|e| format!("Failed to serialize schema: {e}"))?;

    if stdout {
        println!("{json}");
    } else {
        let config_dir = ConfigPaths::config_dir()
            .map_err(|e| format!("Failed to determine config directory: {e}"))?;

        let schema_path = config_dir.join("schema.json");
        fs::write(&schema_path, &json)
            .map_err(|e| format!("Failed to write schema: {e}"))?;

        let taplo_path = config_dir.join(".taplo.toml");
        fs::write(&taplo_path, TAPLO_CONFIG)
            .map_err(|e| format!("Failed to write .taplo.toml: {e}"))?;

        println!("Written:");
        println!("  {}", schema_path.display());
        println!("  {}", taplo_path.display());
    }

    Ok(())
}
