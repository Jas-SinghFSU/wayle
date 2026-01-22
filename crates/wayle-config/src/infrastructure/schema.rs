//! JSON Schema generation for configuration validation and editor support.

use std::{fs, io};

use schemars::schema_for;
use tracing::{debug, info};

use super::paths::ConfigPaths;
use crate::Config;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Generates the JSON Schema for Wayle's configuration.
///
/// The schema includes the package version in the `$id` field for version tracking.
pub fn generate_schema() -> String {
    let schema = schema_for!(Config);
    let mut json: serde_json::Value =
        serde_json::to_value(&schema).expect("schema serialization cannot fail");

    if let Some(obj) = json.as_object_mut() {
        obj.insert(
            "$id".to_string(),
            serde_json::Value::String(format!("wayle-config-{VERSION}")),
        );
    }

    serde_json::to_string_pretty(&json).expect("JSON serialization cannot fail")
}

const TAPLO_CONFIG: &str = r#"[schema]
path = "./schema.json"
"#;

/// Ensures the schema and Taplo config files exist and are up-to-date.
///
/// Writes `schema.json` and `.taplo.toml` to `~/.config/wayle/` if:
/// - The files don't exist
/// - The schema exists but contains a different version
///
/// # Errors
///
/// Returns error if the files cannot be written.
pub fn ensure_schema_current() -> io::Result<()> {
    let schema_path = ConfigPaths::schema_json();
    let taplo_path = ConfigPaths::taplo_config();

    if let Some(parent) = schema_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    let needs_update = match fs::read_to_string(&schema_path) {
        Ok(existing) => !existing.contains(&format!("wayle-config-{VERSION}")),
        Err(_) => true,
    };

    if needs_update {
        let schema = generate_schema();
        fs::write(&schema_path, schema)?;
        info!(path = %schema_path.display(), version = VERSION, "Schema generated");
    } else {
        debug!(path = %schema_path.display(), "Schema already current");
    }

    if !taplo_path.exists() {
        fs::write(&taplo_path, TAPLO_CONFIG)?;
        info!(path = %taplo_path.display(), "Taplo config generated");
    }

    Ok(())
}

/// Writes the schema to the specified path, regardless of version.
///
/// # Errors
///
/// Returns error if the schema file cannot be written.
pub fn write_schema_to(path: &std::path::Path) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    let schema = generate_schema();
    fs::write(path, schema)
}
