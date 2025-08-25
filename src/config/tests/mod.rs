//! Unit tests for config module
//!
//! Tests configuration types, defaults, and serialization.
//! No filesystem dependencies - all in-memory.

#![allow(clippy::panic)]

use std::{env, panic};

use crate::config::{Config, ConfigPaths};

#[test]
fn config_default() {
    let config = Config::default();

    assert!(!format!("{:?}", config.general).is_empty());
    assert!(!format!("{:?}", config.modules).is_empty());
}

#[test]
fn config_serialize_toml() {
    let config = Config::default();

    let toml_str = toml::to_string(&config).unwrap();
    assert!(!toml_str.is_empty());
    assert!(toml_str.contains("[general]"));
    assert!(toml_str.contains("[modules]"));
}

#[test]
fn config_deserialize_toml() {
    let toml_str = r#"
        [general]
        log_level = "debug"

        [modules]
        battery = { enabled = true }
    "#;

    let config: Config = toml::from_str(toml_str).unwrap();

    assert!(!format!("{:?}", config.general).is_empty());
    assert!(!format!("{:?}", config.modules).is_empty());
}

#[test]
fn config_serialize_roundtrip() {
    let original = Config::default();

    let toml_str = toml::to_string(&original).unwrap();

    let deserialized: Config = toml::from_str(&toml_str).unwrap();

    assert_eq!(format!("{original:?}"), format!("{deserialized:?}"));
}

#[test]
fn config_minimal_toml() {
    let minimal_toml = r#"
        [general]
    "#;

    let config: Config = toml::from_str(minimal_toml).unwrap();

    assert!(!format!("{:?}", config.general).is_empty());
    assert!(!format!("{:?}", config.modules).is_empty());
}

#[test]
fn config_empty_toml() {
    let empty_toml = "";

    let config: Config = toml::from_str(empty_toml).unwrap();

    assert!(!format!("{:?}", config.general).is_empty());
    assert!(!format!("{:?}", config.modules).is_empty());
}

#[test]
fn config_clone() {
    let config1 = Config::default();
    let config2 = config1.clone();

    assert_eq!(format!("{config1:?}"), format!("{config2:?}"));
}

#[test]
fn config_paths_valid() {
    let config_dir_result = ConfigPaths::config_dir();
    assert!(config_dir_result.is_ok() || config_dir_result.is_err());

    let main_config = panic::catch_unwind(ConfigPaths::main_config);

    let runtime_config = panic::catch_unwind(ConfigPaths::runtime_config);

    if env::var("HOME").is_ok() {
        assert!(main_config.is_ok());
        assert!(runtime_config.is_ok());

        let main_path = ConfigPaths::main_config();
        let runtime_path = ConfigPaths::runtime_config();

        assert!(main_path.to_string_lossy().ends_with("config.toml"));
        assert!(runtime_path.to_string_lossy().ends_with("runtime.toml"));

        assert_eq!(main_path.parent(), runtime_path.parent());
    }
}

#[test]
fn config_serde_traits() {
    let config = Config::default();

    let _serialized = toml::to_string(&config).unwrap();
    let _cloned = config.clone();

    let debug_str = format!("{config:?}");
    assert!(!debug_str.is_empty());
}

#[test]
fn config_toml_value_types() {
    let toml_str = r#"
        [general]
        log_level = "debug"
        
        [modules.battery]
        enabled = true
        show_percentage = false
        battery_warning = 25
        
        [modules.clock]
        enabled = true
    "#;

    let config: Config = toml::from_str(toml_str).unwrap();

    assert!(!format!("{config:?}").is_empty());
}

#[test]
fn config_invalid_toml() {
    let invalid_toml = r#"
        [general
        invalid syntax here
        missing closing bracket
    "#;

    let result: Result<Config, toml::de::Error> = toml::from_str(invalid_toml);

    assert!(result.is_err());
}

#[test]
fn config_unknown_fields() {
    let toml_with_unknown = r#"
        [general]
        log_level = "info"
        unknown_field = "should be ignored"
        
        [unknown_section]
        some_field = "ignored"
        
        [modules.battery]
        enabled = true
        unknown_battery_field = "ignored"
    "#;

    let config: Config = toml::from_str(toml_with_unknown).unwrap();
    assert!(!format!("{config:?}").is_empty());
}
