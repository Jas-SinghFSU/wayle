use super::{
    generator::DocsError,
    module::{ModuleInfo, SchemeFn},
    schema::{PropertyInfo, extract_property_info},
};

const TABLE_HEADER: &str =
    "| Property | Type | Description | Default |\n|----------|------|-------------|---------|";

/// Generates a markdown table documenting configuration properties.
///
/// Creates a formatted table with property names, types, descriptions,
/// and default values for display in documentation.
pub fn generate_property_table(
    section_title: &str,
    config_path: &str,
    properties: Vec<PropertyInfo>,
) -> String {
    if properties.is_empty() {
        return String::new();
    }

    let property_rows = properties
        .iter()
        .map(|prop| {
            format!(
                "| `{}` | `{}` | {} | `{}` |",
                prop.name, prop.type_name, prop.description, prop.default_value
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        "## {section_title}\n**Config path:** `{config_path}`\n\n{TABLE_HEADER}\n{property_rows}\n"
    )
}

/// Generates a complete markdown documentation page for a module.
///
/// Creates a structured document including module header, behavior
/// configuration sections, and styling configuration sections.
///
/// # Errors
///
/// Returns `DocsError::SchemaConversion` if schema serialization fails.
pub fn generate_module_page(module: &ModuleInfo) -> Result<String, DocsError> {
    let mut content = String::new();

    content.push_str(&generate_header(module));
    content.push_str(&generate_behavior_sections(module)?);
    content.push_str(&generate_styling_sections(module)?);

    Ok(content)
}

fn generate_header(module: &ModuleInfo) -> String {
    format!(
        "# {} {} Module\n\n{}\n\n",
        module.icon,
        title_case(&module.name),
        module.description
    )
}

fn generate_behavior_sections(module: &ModuleInfo) -> Result<String, DocsError> {
    generate_sections(&module.behavior_configs, &module.name, "Behavior", "")
}

fn generate_styling_sections(module: &ModuleInfo) -> Result<String, DocsError> {
    generate_sections(&module.styling_configs, &module.name, "Styling", ".styling")
}

fn generate_sections(
    configs: &[(String, SchemeFn)],
    module_name: &str,
    section_type: &str,
    path_prefix: &str,
) -> Result<String, DocsError> {
    let mut content = String::new();

    for (config_name, schema_fn) in configs {
        let schema_value =
            serde_json::to_value(schema_fn()).map_err(|e| DocsError::SchemaConversionError {
                module: module_name.to_string(),
                details: format!("Failed to generate section for '{config_name}': {e}"),
            })?;

        let properties = extract_property_info(&schema_value);

        if !properties.is_empty() {
            let section_title = format!("{} {}", title_case(config_name), section_type);

            let config_path = format!("[modules.{module_name}{path_prefix}.{config_name}]");

            let generated_table = generate_property_table(&section_title, &config_path, properties);

            content.push_str(&generated_table);
            content.push('\n');
        }
    }

    Ok(content)
}

fn title_case(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let mut chars = s.chars();
    chars
        .next()
        .unwrap_or_default()
        .to_uppercase()
        .chain(chars.as_str().chars())
        .collect()
}
