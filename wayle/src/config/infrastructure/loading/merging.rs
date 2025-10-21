use toml::{Value, map::Map};

/// Merges multiple TOML configs with the main configuration taking precedence
pub fn merge_toml_configs(imports: Vec<Value>, main: Value) -> Value {
    let mut accumulated = Value::Table(Map::new());

    for import in imports {
        accumulated = merge_two_toml_configs(accumulated, import);
    }

    merge_two_toml_configs(accumulated, main)
}

/// Deep merges two TOML configs while preserving precedence
///
/// We start with overlay as base, then selectively add missing
/// keys from base. This ensures overlay values always win, but we don't lose
/// base values that aren't being overridden. For non-table values, overlay
/// completely replaces base (no attempt to merge primitives).
pub fn merge_two_toml_configs(base: Value, overlay: Value) -> Value {
    match (base, overlay) {
        (Value::Table(base_table), Value::Table(overlay_table)) => {
            let mut merged_table = overlay_table;

            for (key, base_value) in base_table {
                match merged_table.remove(&key) {
                    None => {
                        merged_table.insert(key, base_value);
                    }
                    Some(overlay_value) => {
                        let merged_value = merge_two_toml_configs(base_value, overlay_value);
                        merged_table.insert(key, merged_value);
                    }
                }
            }

            Value::Table(merged_table)
        }
        (_, overlay) => overlay,
    }
}
