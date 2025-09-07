use std::{collections::HashSet, error::Error, sync::OnceLock, time::Instant};

use toml::Value;

use crate::{
    config::Config,
    config_runtime::{
        changes::{ConfigChange, ConfigError},
        path_ops::navigate_path,
    },
};

static DEFAULT_CONFIG: OnceLock<toml::Value> = OnceLock::new();

/// Compares two configurations and identifies specific field-level changes.
///
/// Performs a recursive comparison of configuration structures by converting them
/// to TOML and analyzing differences at the field level. When fields are removed
/// from the new config, their default values are automatically provided.
///
/// # Arguments
/// * `old` - The previous configuration state
/// * `new` - The new configuration state to compare against
///
/// # Errors
/// Returns error if either configuration cannot be serialized to TOML format.
pub fn diff_configs(old: &Config, new: &Config) -> Result<Vec<ConfigChange>, Box<dyn Error>> {
    let old_toml = toml::to_string(old)?;
    let new_toml = toml::to_string(new)?;

    let old_value: toml::Value = toml::from_str(&old_toml)?;
    let new_value: toml::Value = toml::from_str(&new_toml)?;

    let changes = diff_toml_values("", &old_value, &new_value, Instant::now());

    Ok(changes)
}

fn diff_toml_values(
    path: &str,
    old: &toml::Value,
    new: &toml::Value,
    timestamp: Instant,
) -> Vec<ConfigChange> {
    let mut changes = Vec::new();

    match (old, new) {
        (Value::Table(old_table), Value::Table(new_table)) => {
            let mut all_keys = HashSet::new();
            all_keys.extend(old_table.keys());
            all_keys.extend(new_table.keys());

            for key in all_keys {
                let field_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{path}.{key}")
                };

                match (old_table.get(key), new_table.get(key)) {
                    (Some(old_val), Some(new_val)) => {
                        changes.extend(handle_value_changed(
                            &field_path,
                            old_val,
                            new_val,
                            timestamp,
                        ));
                    }
                    (Some(old_val), None) => {
                        if let Some(change) = handle_value_removed(&field_path, old_val, timestamp)
                        {
                            changes.push(change);
                        }
                    }
                    (None, Some(new_val)) => {
                        changes.push(handle_value_added(&field_path, new_val, timestamp));
                    }
                    (None, None) => unreachable!(),
                }
            }
        }
        _ => {
            if old != new {
                changes.push(ConfigChange {
                    path: path.to_string(),
                    old_value: Some(old.clone()),
                    new_value: new.clone(),
                    timestamp,
                });
            }
        }
    }

    changes
}

#[allow(clippy::expect_used)]
fn get_default_config() -> &'static toml::Value {
    DEFAULT_CONFIG.get_or_init(|| {
        toml::Value::try_from(Config::default()).expect("Default config must be serializable")
    })
}

fn get_default_for_path(path: &str) -> Result<toml::Value, ConfigError> {
    let default_config = get_default_config();
    navigate_path(default_config, path)
}

fn handle_value_changed(
    path: &str,
    old_val: &toml::Value,
    new_val: &toml::Value,
    timestamp: Instant,
) -> Vec<ConfigChange> {
    diff_toml_values(path, old_val, new_val, timestamp)
}

fn handle_value_removed(
    path: &str,
    old_val: &toml::Value,
    timestamp: Instant,
) -> Option<ConfigChange> {
    get_default_for_path(path)
        .ok()
        .map(|default_value| ConfigChange {
            path: path.to_string(),
            old_value: Some(old_val.clone()),
            new_value: default_value,
            timestamp,
        })
}

fn handle_value_added(path: &str, new_val: &toml::Value, timestamp: Instant) -> ConfigChange {
    ConfigChange {
        path: path.to_string(),
        old_value: None,
        new_value: new_val.clone(),
        timestamp,
    }
}
