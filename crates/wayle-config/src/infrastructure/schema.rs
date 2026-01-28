//! JSON Schema generation for configuration validation and editor support.

use std::{fs, io, path::Path};

use schemars::generate::{SchemaGenerator, SchemaSettings};
use tracing::{debug, info};

use super::paths::ConfigPaths;
use crate::Config;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Generates the JSON Schema for Wayle's configuration.
///
/// The schema includes the package version in the `$id` field for version tracking.
/// Uses `inline_subschemas` to ensure field descriptions are visible in TOML editors.
///
/// Returns `None` if schema serialization fails (should never occur).
pub fn generate_schema() -> Option<String> {
    let settings = SchemaSettings::default().with(|s| {
        s.inline_subschemas = true;
    });
    let generator = SchemaGenerator::new(settings);
    let schema = generator.into_root_schema_for::<Config>();

    let mut json: serde_json::Value = match serde_json::to_value(&schema) {
        Ok(v) => v,
        Err(e) => {
            tracing::error!(error = %e, "failed to serialize schema to JSON value");
            return None;
        }
    };

    if let Some(obj) = json.as_object_mut() {
        obj.insert(
            "$id".to_string(),
            serde_json::Value::String(format!("wayle-config-{VERSION}")),
        );
    }

    match serde_json::to_string_pretty(&json) {
        Ok(s) => Some(s),
        Err(e) => {
            tracing::error!(error = %e, "failed to serialize JSON to string");
            None
        }
    }
}

const TOMBI_CONFIG: &str = r#"[schema]
enabled = true

[[schemas]]
path = "./schema.json"
include = ["*.toml"]
"#;

/// Ensures the schema and Tombi config files exist and are up-to-date.
///
/// Writes `schema.json` and `tombi.toml` to `~/.config/wayle/` if:
/// - The files don't exist
/// - The schema exists but contains a different version
///
/// # Errors
///
/// Returns error if the files cannot be written or schema generation fails.
pub fn ensure_schema_current() -> io::Result<()> {
    let schema_path = ConfigPaths::schema_json();
    let tombi_path = ConfigPaths::tombi_config();

    if let Some(parent) = schema_path.parent()
        && !parent.exists()
    {
        fs::create_dir_all(parent)?;
    }

    let needs_update = match fs::read_to_string(&schema_path) {
        Ok(existing) => !existing.contains(&format!("wayle-config-{VERSION}")),
        Err(_) => true,
    };

    if needs_update {
        let Some(schema) = generate_schema() else {
            return Err(io::Error::other("schema generation failed"));
        };
        fs::write(&schema_path, schema)?;
        info!(path = %schema_path.display(), version = VERSION, "Schema generated");
    } else {
        debug!(path = %schema_path.display(), "Schema already current");
    }

    if !tombi_path.exists() {
        fs::write(&tombi_path, TOMBI_CONFIG)?;
        info!(path = %tombi_path.display(), "Tombi config generated");
    }

    Ok(())
}

/// Writes the schema to the specified path, regardless of version.
///
/// # Errors
///
/// Returns error if the schema file cannot be written or schema generation fails.
pub fn write_schema_to(path: &Path) -> io::Result<()> {
    if let Some(parent) = path.parent()
        && !parent.exists()
    {
        fs::create_dir_all(parent)?;
    }

    let Some(schema) = generate_schema() else {
        return Err(io::Error::other("schema generation failed"));
    };
    fs::write(path, schema)
}
