//! Template engine for format strings using minijinja.
//!
//! # Syntax
//!
//! Uses Jinja2 template syntax:
//! - `{{ variable }}` for variable substitution
//! - `{{ variable | default('fallback') }}` for fallback values
//! - `{{ "%02d" | format(count) }}` for zero-padding
//! - `{{ name | upper }}`, `| lower`, `| trim` for string transforms

use minijinja::Error;
pub use minijinja::{Environment, Error as TemplateError, ErrorKind, Value};

/// Renders a template string with the given context value.
///
/// The context should be a serde-serializable value (typically a `serde_json::Value`
/// or a struct). All fields become template variables.
///
/// # Errors
///
/// Returns error on template syntax errors or render failures.
pub fn render(template: &str, context: impl serde::Serialize) -> Result<String, Error> {
    let env = Environment::new();
    env.render_str(template, context)
}

/// Renders a template with custom environment configuration.
///
/// The `configure` closure receives a mutable reference to the [`Environment`],
/// allowing registration of custom functions, filters, or globals before rendering.
///
/// # Errors
///
/// Returns error on template syntax errors or render failures.
pub fn render_with<F>(
    template: &str,
    context: impl serde::Serialize,
    configure: F,
) -> Result<String, Error>
where
    F: FnOnce(&mut Environment<'_>),
{
    let mut env = Environment::new();
    configure(&mut env);
    env.render_str(template, context)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn simple_variable() {
        let ctx = json!({"name": "world"});
        let result = render("hello {{ name }}", ctx).unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn default_filter_on_missing() {
        let ctx = json!({});
        let result = render("{{ missing | default('fallback') }}", ctx).unwrap();
        assert_eq!(result, "fallback");
    }

    #[test]
    fn default_filter_on_present() {
        let ctx = json!({"text": "actual"});
        let result = render("{{ text | default('fallback') }}", ctx).unwrap();
        assert_eq!(result, "actual");
    }

    #[test]
    fn zero_padding_via_format() {
        let ctx = json!({"count": 5});
        let result = render(r#"{{ "%02d" | format(count) }}"#, ctx).unwrap();
        assert_eq!(result, "05");
    }

    #[test]
    fn upper_filter() {
        let ctx = json!({"name": "hello"});
        let result = render("{{ name | upper }}", ctx).unwrap();
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn nested_dot_access() {
        let ctx = json!({"data": {"temp": 22}});
        let result = render("Temp: {{ data.temp }}C", ctx).unwrap();
        assert_eq!(result, "Temp: 22C");
    }

    #[test]
    fn conditional_expression() {
        let ctx = json!({"count": 0});
        let result = render("{{ 'none' if count == 0 else count }}", ctx).unwrap();
        assert_eq!(result, "none");
    }

    #[test]
    fn mixed_template() {
        let ctx = json!({"output": "42", "status": "ok"});
        let result = render("Value: {{ output }} [{{ status | upper }}]", ctx).unwrap();
        assert_eq!(result, "Value: 42 [OK]");
    }

    #[test]
    fn literal_braces() {
        let ctx = json!({});
        let result = render("{{ '{{' }}escaped{{ '}}' }}", ctx).unwrap();
        assert_eq!(result, "{{escaped}}");
    }

    #[test]
    fn empty_template() {
        let ctx = json!({});
        let result = render("", ctx).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn plain_text_no_expressions() {
        let ctx = json!({});
        let result = render("just plain text", ctx).unwrap();
        assert_eq!(result, "just plain text");
    }

    #[test]
    fn chained_filters() {
        let ctx = json!({"name": "  hello world  "});
        let result = render("{{ name | trim | upper }}", ctx).unwrap();
        assert_eq!(result, "HELLO WORLD");
    }
}
