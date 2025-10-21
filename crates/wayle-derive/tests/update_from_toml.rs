#![allow(missing_docs)]

use wayle_common::{Property, UpdateFromToml};
use wayle_derive::UpdateFromToml;

#[derive(UpdateFromToml)]
struct SimpleConfig {
    enabled: Property<bool>,
    count: Property<u32>,
}

#[test]
fn updates_all_fields_from_table() {
    let config = SimpleConfig {
        enabled: Property::new(false),
        count: Property::new(0),
    };

    let toml_value = toml::toml! {
        enabled = true
        count = 42
    };

    config.update_from_toml(&toml::Value::Table(toml_value));

    assert!(config.enabled.get());
    assert_eq!(config.count.get(), 42);
}

#[test]
fn updates_partial_fields() {
    let config = SimpleConfig {
        enabled: Property::new(false),
        count: Property::new(10),
    };

    let toml_value = toml::toml! {
        enabled = true
    };

    config.update_from_toml(&toml::Value::Table(toml_value));

    assert!(config.enabled.get());
    assert_eq!(config.count.get(), 10);
}

#[test]
fn ignores_unknown_fields() {
    let config = SimpleConfig {
        enabled: Property::new(false),
        count: Property::new(0),
    };

    let toml_value = toml::toml! {
        enabled = true
        unknown_field = "ignored"
        count = 99
    };

    config.update_from_toml(&toml::Value::Table(toml_value));

    assert!(config.enabled.get());
    assert_eq!(config.count.get(), 99);
}

#[test]
fn handles_non_table_value() {
    let config = SimpleConfig {
        enabled: Property::new(true),
        count: Property::new(5),
    };

    config.update_from_toml(&toml::Value::String("not a table".to_string()));

    assert!(config.enabled.get());
    assert_eq!(config.count.get(), 5);
}

#[test]
fn handles_empty_table() {
    let config = SimpleConfig {
        enabled: Property::new(true),
        count: Property::new(5),
    };

    use toml::map::Map;
    let toml_value = Map::new();

    config.update_from_toml(&toml::Value::Table(toml_value));

    assert!(config.enabled.get());
    assert_eq!(config.count.get(), 5);
}

#[derive(UpdateFromToml)]
struct NestedConfig {
    simple: SimpleConfig,
    name: Property<String>,
}

#[test]
fn updates_nested_structs() {
    let config = NestedConfig {
        simple: SimpleConfig {
            enabled: Property::new(false),
            count: Property::new(0),
        },
        name: Property::new("old".to_string()),
    };

    let toml_value = toml::toml! {
        name = "new"
        [simple]
        enabled = true
        count = 100
    };

    config.update_from_toml(&toml::Value::Table(toml_value));

    assert_eq!(config.name.get(), "new");
    assert!(config.simple.enabled.get());
    assert_eq!(config.simple.count.get(), 100);
}
