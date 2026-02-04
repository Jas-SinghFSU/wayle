use crate::i18n::t;

fn format_duration(total_secs: u32) -> String {
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

pub(crate) struct LabelContext {
    pub active: bool,
    pub duration_mins: u32,
    pub remaining_secs: Option<u32>,
}

pub(super) fn build_label(format: &str, ctx: &LabelContext) -> String {
    let state = if ctx.active {
        t!("bar-idle-inhibit-on")
    } else {
        t!("bar-idle-inhibit-off")
    };
    let remaining = if !ctx.active {
        String::from("-")
    } else {
        ctx.remaining_secs
            .map_or_else(|| String::from("∞"), format_duration)
    };
    let duration = if ctx.duration_mins == 0 {
        String::from("∞")
    } else {
        format!("{}", ctx.duration_mins)
    };

    format
        .replace("{state}", &state)
        .replace("{remaining}", &remaining)
        .replace("{duration}", &duration)
}

/// Selects icon based on active state.
pub(super) fn select_icon(active: bool, icon_inactive: &str, icon_active: &str) -> String {
    if active {
        icon_active.to_string()
    } else {
        icon_inactive.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_duration_seconds_only() {
        assert_eq!(format_duration(45), "0:45");
    }

    #[test]
    fn format_duration_minutes_seconds() {
        assert_eq!(format_duration(125), "2:05");
        assert_eq!(format_duration(3599), "59:59");
    }

    #[test]
    fn format_duration_with_hours() {
        assert_eq!(format_duration(3600), "1:00:00");
        assert_eq!(format_duration(3661), "1:01:01");
    }

    fn ctx(active: bool, duration_mins: u32, remaining_secs: Option<u32>) -> LabelContext {
        LabelContext {
            active,
            duration_mins,
            remaining_secs,
        }
    }

    #[test]
    fn build_label_state_on() {
        assert_eq!(build_label("{state}", &ctx(true, 0, None)), "On");
    }

    #[test]
    fn build_label_state_off() {
        assert_eq!(build_label("{state}", &ctx(false, 0, None)), "Off");
    }

    #[test]
    fn build_label_remaining() {
        assert_eq!(
            build_label("{remaining}", &ctx(true, 30, Some(125))),
            "2:05"
        );
    }

    #[test]
    fn build_label_duration_timed() {
        assert_eq!(build_label("{duration}", &ctx(true, 30, None)), "30");
    }

    #[test]
    fn build_label_duration_indefinite() {
        assert_eq!(build_label("{duration}", &ctx(true, 0, None)), "∞");
    }

    #[test]
    fn build_label_all_placeholders() {
        assert_eq!(
            build_label(
                "{state}: {remaining} / {duration}",
                &ctx(true, 60, Some(1800))
            ),
            "On: 30:00 / 60"
        );
    }

    #[test]
    fn build_label_inactive_shows_dash() {
        assert_eq!(build_label("{remaining}", &ctx(false, 30, None)), "-");
    }

    #[test]
    fn build_label_active_indefinite_shows_infinity() {
        assert_eq!(build_label("{remaining}", &ctx(true, 0, None)), "∞");
    }

    #[test]
    fn select_icon_inactive() {
        assert_eq!(select_icon(false, "off", "on"), "off");
    }

    #[test]
    fn select_icon_active() {
        assert_eq!(select_icon(true, "off", "on"), "on");
    }
}
