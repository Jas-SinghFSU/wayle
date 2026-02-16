use gtk4::glib::{DateTime, TimeZone};
use tracing::warn;
use wayle_common::template::{ErrorKind, TemplateError, Value};

pub(super) fn format_world_clock(format: &str) -> String {
    wayle_common::template::render_with(format, (), |env| {
        env.add_function("tz", tz_function);
    })
    .unwrap_or_default()
}

fn tz_function(tz_id: &str, time_format: &str) -> Result<Value, TemplateError> {
    let tz = TimeZone::from_identifier(Some(tz_id)).ok_or_else(|| {
        warn!(timezone = %tz_id, "invalid timezone identifier");
        TemplateError::new(
            ErrorKind::InvalidOperation,
            format!("invalid timezone: {tz_id}"),
        )
    })?;

    let formatted = DateTime::now(&tz)
        .ok()
        .and_then(|dt| dt.format(time_format).ok())
        .map(String::from)
        .unwrap_or_default();

    Ok(Value::from(formatted))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_returns_empty() {
        assert_eq!(format_world_clock(""), "");
    }

    #[test]
    fn plain_text_preserved() {
        assert_eq!(format_world_clock("NYC  TYO"), "NYC  TYO");
    }

    #[test]
    fn valid_timezone_formatted() {
        assert_eq!(format_world_clock("{{ tz('UTC', '%Z') }}"), "UTC");
    }

    #[test]
    fn multiple_timezones_all_formatted() {
        assert_eq!(
            format_world_clock("{{ tz('UTC', '%Z') }} | {{ tz('UTC', '%Z') }}"),
            "UTC | UTC"
        );
    }

    #[test]
    fn mixed_text_and_timezones() {
        assert_eq!(
            format_world_clock("Time: {{ tz('UTC', '%Z') }} end"),
            "Time: UTC end"
        );
    }
}
