use wayle_config::{ConfigPaths, generate_schema, infrastructure::schema};

use crate::cli::CliAction;

/// Generates JSON Schema and Tombi config for editor intellisense.
///
/// Writes `schema.json` and `tombi.toml` to `~/.config/wayle/`.
/// Use `stdout` flag to print schema to terminal instead.
///
/// # Errors
///
/// Returns error if schema serialization or file write fails.
pub fn execute(stdout: bool) -> CliAction {
    if stdout {
        println!("{}", generate_schema());
        return Ok(());
    }

    schema::ensure_schema_current().map_err(|e| format!("Failed to write schema files: {e}"))?;

    println!("Written:");
    println!("  {}", ConfigPaths::schema_json().display());
    println!("  {}", ConfigPaths::tombi_config().display());

    Ok(())
}
