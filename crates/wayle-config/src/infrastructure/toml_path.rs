use super::error::{Error, InvalidFieldReason};

pub(crate) fn insert(root: &mut toml::Value, path: &str, value: toml::Value) -> Result<(), Error> {
    let segments: Vec<&str> = path.split('.').collect();

    let (final_key, parent_segments) =
        segments
            .split_last()
            .ok_or_else(|| Error::InvalidConfigField {
                field: path.to_string(),
                component: String::from("config"),
                reason: InvalidFieldReason::EmptyPath,
            })?;

    let parent = navigate_to_parent(root, parent_segments, path)?;

    let table = parent
        .as_table_mut()
        .ok_or_else(|| Error::InvalidConfigField {
            field: (*final_key).to_string(),
            component: path.to_string(),
            reason: InvalidFieldReason::ParentNotTable,
        })?;

    table.insert((*final_key).to_string(), value);
    Ok(())
}

fn navigate_to_parent<'a>(
    root: &'a mut toml::Value,
    segments: &[&str],
    full_path: &str,
) -> Result<&'a mut toml::Value, Error> {
    let mut current = root;

    for segment in segments {
        let table = current
            .as_table_mut()
            .ok_or_else(|| Error::InvalidConfigField {
                field: (*segment).to_string(),
                component: full_path.to_string(),
                reason: InvalidFieldReason::ParentNotTable,
            })?;

        current = table
            .entry((*segment).to_string())
            .or_insert_with(|| toml::Value::Table(toml::Table::new()));
    }

    Ok(current)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod insert {
        use super::*;

        #[test]
        fn inserts_value_at_simple_path() {
            let mut root = toml::Value::Table(toml::Table::new());
            insert(&mut root, "enabled", toml::Value::Boolean(true)).unwrap();

            let table = root.as_table().unwrap();
            assert_eq!(table.get("enabled"), Some(&toml::Value::Boolean(true)));
        }

        #[test]
        fn inserts_value_at_nested_path() {
            let mut root = toml::Value::Table(toml::Table::new());
            insert(
                &mut root,
                "clock.button.enabled",
                toml::Value::Boolean(true),
            )
            .unwrap();

            let clock = root.get("clock").unwrap().as_table().unwrap();
            let button = clock.get("button").unwrap().as_table().unwrap();
            assert_eq!(button.get("enabled"), Some(&toml::Value::Boolean(true)));
        }

        #[test]
        fn creates_intermediate_tables() {
            let mut root = toml::Value::Table(toml::Table::new());
            insert(&mut root, "a.b.c.value", toml::Value::Integer(42)).unwrap();

            let a = root.get("a").unwrap().as_table().unwrap();
            let b = a.get("b").unwrap().as_table().unwrap();
            let c = b.get("c").unwrap().as_table().unwrap();
            assert_eq!(c.get("value"), Some(&toml::Value::Integer(42)));
        }

        #[test]
        fn overwrites_existing_value() {
            let mut root = toml::Value::Table(toml::Table::new());
            insert(&mut root, "key", toml::Value::Integer(1)).unwrap();
            insert(&mut root, "key", toml::Value::Integer(2)).unwrap();

            let table = root.as_table().unwrap();
            assert_eq!(table.get("key"), Some(&toml::Value::Integer(2)));
        }

        #[test]
        fn inserts_empty_string_key_for_empty_path() {
            let mut root = toml::Value::Table(toml::Table::new());
            insert(&mut root, "", toml::Value::Boolean(true)).unwrap();

            let table = root.as_table().unwrap();
            assert_eq!(table.get(""), Some(&toml::Value::Boolean(true)));
        }

        #[test]
        fn errors_when_parent_is_not_table() {
            let mut root = toml::Value::Table(toml::Table::new());
            insert(&mut root, "key", toml::Value::String("value".into())).unwrap();
            let result = insert(&mut root, "key.nested", toml::Value::Boolean(true));

            assert!(result.is_err());
        }
    }
}
