use toml::Value;

use crate::config_runtime::changes::Error;

pub(super) fn path_matches(path: &str, pattern: &str) -> bool {
    const WILDCARD: &str = "*";

    if pattern == WILDCARD {
        return true;
    };

    let path_parts: Vec<&str> = path.split(".").collect();
    let pattern_parts: Vec<&str> = pattern.split(".").collect();

    for (path_part, pattern_part) in path_parts.iter().zip(pattern_parts.iter()) {
        if pattern_part == &WILDCARD {
            continue;
        }

        if path_part != pattern_part {
            return false;
        }
    }

    true
}

pub(super) fn navigate_path(value: &Value, path: &str) -> Result<Value, Error> {
    let parts: Vec<&str> = path.split(".").collect();
    let mut current = value;

    for (i, part) in parts.iter().enumerate() {
        match current {
            Value::Table(table) => {
                current = table.get(*part).ok_or_else(|| {
                    Error::InvalidPath(format!(
                        "Key '{}' not found in table at path '{}'",
                        part,
                        parts[..i].join(".")
                    ))
                })?;
            }
            Value::Array(array) => {
                let index = part.parse::<usize>().map_err(|e| {
                    Error::InvalidPath(format!(
                        "Invalid array index '{}' at path '{}': {}",
                        part,
                        parts[..i].join("."),
                        e
                    ))
                })?;

                current = array.get(index).ok_or_else(|| {
                    Error::InvalidPath(format!(
                        "Array index '{}' out of bounds at path '{}'",
                        index,
                        parts[..i].join(".")
                    ))
                })?;
            }
            _ => {
                return Err(Error::InvalidPath(format!(
                    "Cannot navigate into {:?} at path '{}'",
                    current.type_str(),
                    parts[..i].join("."),
                )));
            }
        }
    }

    Ok(current.clone())
}

pub(super) fn set_value_at_path(
    value: &mut Value,
    path: &str,
    new_value: Value,
) -> Result<(), Error> {
    let parts: Vec<&str> = path.split('.').collect();

    if parts.is_empty() {
        return Err(Error::InvalidPath(String::from("Empty path")));
    }

    let (parent, last_key) = navigate_to_parent_mut(value, &parts)?;

    insert_value(parent, last_key, new_value)
}

pub(super) fn navigate_to_parent_mut<'a>(
    value: &'a mut Value,
    parts: &'a [&'a str],
) -> Result<(&'a mut Value, &'a str), Error> {
    let mut current = value;

    for (i, part) in parts[..parts.len() - 1].iter().enumerate() {
        current = navigate_step_mut(current, part, &parts[..=i])?;
    }

    Ok((current, parts[parts.len() - 1]))
}

pub(super) fn navigate_step_mut<'a>(
    current: &'a mut Value,
    key: &str,
    path_so_far: &[&str],
) -> Result<&'a mut Value, Error> {
    match current {
        Value::Table(table) => {
            if !table.contains_key(key) {
                table.insert(String::from(key), Value::Table(toml::map::Map::new()));
            }

            table.get_mut(key).ok_or_else(|| {
                Error::InvalidPath(format!(
                    "Key '{}' not found at path '{}'",
                    key,
                    path_so_far.join(".")
                ))
            })
        }
        Value::Array(arr) => {
            let index = key.parse::<usize>().map_err(|e| {
                Error::InvalidPath(format!(
                    "Invalid array index '{}' at path '{}': {}",
                    key,
                    path_so_far.join("."),
                    e
                ))
            })?;

            arr.get_mut(index).ok_or_else(|| {
                Error::InvalidPath(format!(
                    "Array index {} out of bounds at path '{}'",
                    index,
                    path_so_far.join(".")
                ))
            })
        }
        _ => Err(Error::InvalidPath(format!(
            "Cannot navigate into {} at path '{}'",
            current.type_str(),
            path_so_far.join(".")
        ))),
    }
}

pub(super) fn insert_value(
    container: &mut Value,
    key: &str,
    new_value: Value,
) -> Result<(), Error> {
    match container {
        Value::Table(table) => {
            table.insert(String::from(key), new_value);
            Ok(())
        }
        Value::Array(arr) => {
            let index = key
                .parse::<usize>()
                .map_err(|e| Error::InvalidPath(format!("Invalid array index '{key}': {e}")))?;

            arr.get_mut(index)
                .map(|elem| *elem = new_value)
                .ok_or_else(|| Error::InvalidPath(format!("Array index {index} out of bounds")))
        }
        _ => Err(Error::InvalidPath(format!(
            "Cannot insert into {}",
            container.type_str()
        ))),
    }
}
