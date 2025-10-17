//! Schema parsing and property extraction utilities for JSON Schema documents.
//!
//! This module provides functionality to parse JSON Schema documents and extract
//! property information for documentation generation purposes.

use serde_json::{Map, Value};

/// Represents information about a single property in a JSON Schema.
///
/// This struct captures the essential metadata of a schema property including
/// its name, type, description, and default value for documentation purposes.
#[derive(Debug, Clone)]
pub struct PropertyInfo {
    /// The name of the property as defined in the schema.
    pub name: String,
    /// The JSON Schema type of the property (e.g., "string", "number", "boolean").
    pub type_name: String,
    /// Human-readable description of the property's purpose and usage.
    pub description: String,
    /// String representation of the property's default value, or "-" if not specified.
    pub default_value: String,
}

/// Extracts property information from a JSON Schema document.
///
/// Parses the "properties" object from a JSON Schema and transforms each property
/// into a structured `PropertyInfo` representation suitable for documentation generation.
///
/// # Arguments
///
/// * `schema` - A JSON Schema document as a serde_json Value
///
/// # Returns
///
/// A vector of `PropertyInfo` structs representing all properties found in the schema.
/// Returns an empty vector if no properties are found or the schema is invalid.
pub fn extract_property_info(schema: &Value) -> Vec<PropertyInfo> {
    schema
        .get("properties")
        .and_then(|props| props.as_object())
        .map(build_properties)
        .unwrap_or_default()
}

fn build_properties(props_obj: &Map<String, Value>) -> Vec<PropertyInfo> {
    props_obj
        .iter()
        .map(|(name, property)| PropertyInfo {
            name: name.clone(),
            type_name: get_type(property),
            description: get_description(property),
            default_value: get_default_value(property),
        })
        .collect()
}

fn get_type(property: &Value) -> String {
    property
        .get("type")
        .and_then(|type_of| type_of.as_str())
        .unwrap_or("unknown")
        .to_string()
}

fn get_description(property: &Value) -> String {
    property
        .get("description")
        .and_then(|desc| desc.as_str())
        .unwrap_or("No description provided")
        .to_string()
}

fn get_default_value(property: &Value) -> String {
    property
        .get("default")
        .map(|def_val| match def_val {
            Value::String(s) => format!("\"{s}\""),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            _ => def_val.to_string(),
        })
        .unwrap_or(String::from("-"))
}
