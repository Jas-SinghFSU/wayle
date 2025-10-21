use std::collections::HashMap;

use super::error::Error;

/// Insert a value at a dot-separated path in a TOML structure.
///
/// Creates intermediate tables as needed.
///
/// # Arguments
///
/// * `root` - The root TOML value to modify
/// * `path` - Dot-separated path (e.g., "battery.enabled")
/// * `value` - The value to insert
///
/// # Errors
///
/// Returns error if path is invalid or intermediate values are not tables.
pub fn insert(root: &mut toml::Value, path: &str, value: toml::Value) -> Result<(), Error> {
    let segments: Vec<&str> = path.split('.').collect();

    let (final_key, parent_segments) = segments.split_last().ok_or_else(|| {
        Error::InvalidConfigField {
            field: path.to_string(),
            component: String::from("config"),
            reason: String::from("empty path"),
        }
    })?;

    let parent = navigate_to_parent(root, parent_segments, path)?;

    let table = parent.as_table_mut().ok_or_else(|| {
        Error::InvalidConfigField {
            field: (*final_key).to_string(),
            component: path.to_string(),
            reason: String::from("parent is not a table"),
        }
    })?;

    table.insert((*final_key).to_string(), value);
    Ok(())
}

/// Navigate to the parent table for a given path, creating intermediate tables.
///
/// # Arguments
///
/// * `root` - The root TOML value
/// * `segments` - Path segments to navigate through
/// * `full_path` - Full path for error messages
///
/// # Errors
///
/// Returns error if any intermediate value is not a table.
fn navigate_to_parent<'a>(
    root: &'a mut toml::Value,
    segments: &[&str],
    full_path: &str,
) -> Result<&'a mut toml::Value, Error> {
    let mut current = root;

    for segment in segments {
        let table = current.as_table_mut().ok_or_else(|| {
            Error::InvalidConfigField {
                field: (*segment).to_string(),
                component: full_path.to_string(),
                reason: String::from("parent is not a table"),
            }
        })?;

        current = table
            .entry((*segment).to_string())
            .or_insert_with(|| toml::Value::Table(toml::Table::new()));
    }

    Ok(current)
}

/// Flatten a TOML structure into dot-separated paths.
///
/// Only leaf values (non-tables) are stored in the map.
/// Tables are recursively traversed to build paths.
///
/// # Arguments
///
/// * `value` - The TOML value to flatten
/// * `prefix` - Current path prefix
/// * `map` - HashMap to store flattened paths
pub fn flatten(value: &toml::Value, prefix: &str, map: &mut HashMap<String, toml::Value>) {
    match value {
        toml::Value::Table(table) => {
            for (key, val) in table {
                let path = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{prefix}.{key}")
                };
                flatten(val, &path, map);
            }
        }
        _ => {
            map.insert(prefix.to_string(), value.clone());
        }
    }
}
