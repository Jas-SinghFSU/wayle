use gtk4::glib::{DateTime, TimeZone};
use tracing::warn;

pub(super) fn format_world_clock(format: &str) -> String {
    let mut result = String::with_capacity(format.len());
    let mut chars = format.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch != '{' {
            result.push(ch);
            continue;
        }

        let block: String = chars.by_ref().take_while(|&c| c != '}').collect();
        let formatted = format_timezone_block(&block).unwrap_or_else(|| format!("{{{block}}}"));
        result.push_str(&formatted);
    }

    result
}

fn format_timezone_block(block: &str) -> Option<String> {
    let (tz_id, time_format) = block.split_once(' ')?;

    let tz = TimeZone::from_identifier(Some(tz_id)).or_else(|| {
        warn!(timezone = %tz_id, "invalid timezone identifier");
        None
    })?;

    DateTime::now(&tz)
        .ok()?
        .format(time_format)
        .ok()
        .map(Into::into)
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
        assert_eq!(format_world_clock("{UTC %Z}"), "UTC");
    }

    #[test]
    fn invalid_timezone_preserved_verbatim() {
        assert_eq!(format_world_clock("{InvalidTZ %H:%M}"), "{InvalidTZ %H:%M}");
    }

    #[test]
    fn block_without_format_preserved() {
        assert_eq!(format_world_clock("{UTC}"), "{UTC}");
    }

    #[test]
    fn empty_block_preserved() {
        assert_eq!(format_world_clock("{}"), "{}");
    }

    #[test]
    fn unclosed_brace_still_formats() {
        assert_eq!(format_world_clock("{UTC %Z"), "UTC");
    }

    #[test]
    fn multiple_timezones_all_formatted() {
        assert_eq!(format_world_clock("{UTC %Z} | {UTC %Z}"), "UTC | UTC");
    }

    #[test]
    fn mixed_text_and_timezones() {
        assert_eq!(format_world_clock("Time: {UTC %Z} end"), "Time: UTC end");
    }
}
