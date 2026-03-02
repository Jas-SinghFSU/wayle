use wayle_battery::types::WarningLevel;

pub(super) struct TimeParts {
    pub hours: i64,
    pub minutes: i64,
}

pub(super) fn time_parts(seconds: i64) -> Option<TimeParts> {
    if seconds <= 0 {
        return None;
    }

    Some(TimeParts {
        hours: seconds / 3600,
        minutes: (seconds % 3600) / 60,
    })
}

pub(super) fn format_watts(watts: f64) -> String {
    if watts < 10.0 {
        format!("{watts:.1}W")
    } else {
        format!("{watts:.0}W")
    }
}

pub(super) fn format_watt_hours(wh: f64) -> String {
    if wh < 10.0 {
        format!("{wh:.1} Wh")
    } else {
        format!("{wh:.0} Wh")
    }
}

pub(super) fn gauge_class(percentage: f64, warning_level: &WarningLevel) -> &'static str {
    match warning_level {
        WarningLevel::Low | WarningLevel::Critical | WarningLevel::Action => "crit",
        _ if percentage <= 20.0 => "warn",
        _ => "good",
    }
}

pub(super) fn hero_pct_class(percentage: f64, warning_level: &WarningLevel) -> &'static str {
    match warning_level {
        WarningLevel::Low | WarningLevel::Critical | WarningLevel::Action => "crit",
        _ if percentage <= 20.0 => "warn",
        _ => "",
    }
}

pub(super) fn hero_state_class(warning_level: &WarningLevel, charging: bool) -> &'static str {
    match warning_level {
        WarningLevel::Low | WarningLevel::Critical | WarningLevel::Action => "crit",
        _ if charging => "good",
        _ => "",
    }
}

pub(super) fn health_class(capacity: f64) -> &'static str {
    if capacity <= 0.0 {
        "unknown"
    } else if capacity >= 80.0 {
        "good"
    } else if capacity >= 50.0 {
        "fair"
    } else {
        "poor"
    }
}

pub(super) fn health_value(capacity: f64) -> String {
    if capacity <= 0.0 {
        String::from("--")
    } else {
        format!("{capacity:.0}%")
    }
}

pub(super) fn is_low_battery(warning_level: &WarningLevel) -> bool {
    matches!(
        warning_level,
        WarningLevel::Low | WarningLevel::Critical | WarningLevel::Action
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_parts_hours_and_minutes() {
        let parts = time_parts(12240).unwrap();
        assert_eq!(parts.hours, 3);
        assert_eq!(parts.minutes, 24);
    }

    #[test]
    fn time_parts_minutes_only() {
        let parts = time_parts(1080).unwrap();
        assert_eq!(parts.hours, 0);
        assert_eq!(parts.minutes, 18);
    }

    #[test]
    fn time_parts_zero() {
        assert!(time_parts(0).is_none());
    }

    #[test]
    fn time_parts_negative() {
        assert!(time_parts(-1).is_none());
    }

    #[test]
    fn format_watts_small() {
        assert_eq!(format_watts(8.2), "8.2W");
    }

    #[test]
    fn format_watts_large() {
        assert_eq!(format_watts(45.0), "45W");
    }

    #[test]
    fn format_wh_small() {
        assert_eq!(format_watt_hours(7.2), "7.2 Wh");
    }

    #[test]
    fn format_wh_large() {
        assert_eq!(format_watt_hours(60.0), "60 Wh");
    }

    #[test]
    fn gauge_class_critical_warning_level() {
        assert_eq!(gauge_class(12.0, &WarningLevel::Critical), "crit");
    }

    #[test]
    fn gauge_class_low_percentage() {
        assert_eq!(gauge_class(15.0, &WarningLevel::None), "warn");
    }

    #[test]
    fn gauge_class_normal() {
        assert_eq!(gauge_class(76.0, &WarningLevel::None), "good");
    }

    #[test]
    fn hero_pct_critical() {
        assert_eq!(hero_pct_class(5.0, &WarningLevel::Critical), "crit");
    }

    #[test]
    fn hero_pct_low() {
        assert_eq!(hero_pct_class(15.0, &WarningLevel::None), "warn");
    }

    #[test]
    fn hero_pct_normal() {
        assert_eq!(hero_pct_class(76.0, &WarningLevel::None), "");
    }

    #[test]
    fn hero_state_charging() {
        assert_eq!(hero_state_class(&WarningLevel::None, true), "good");
    }

    #[test]
    fn hero_state_critical() {
        assert_eq!(hero_state_class(&WarningLevel::Critical, false), "crit");
    }

    #[test]
    fn hero_state_normal_discharging() {
        assert_eq!(hero_state_class(&WarningLevel::None, false), "");
    }

    #[test]
    fn health_class_good() {
        assert_eq!(health_class(92.0), "good");
    }

    #[test]
    fn health_class_fair() {
        assert_eq!(health_class(68.0), "fair");
    }

    #[test]
    fn health_class_poor() {
        assert_eq!(health_class(30.0), "poor");
    }

    #[test]
    fn health_class_unavailable() {
        assert_eq!(health_class(0.0), "unknown");
    }

    #[test]
    fn health_value_available() {
        assert_eq!(health_value(92.0), "92%");
    }

    #[test]
    fn health_value_unavailable() {
        assert_eq!(health_value(0.0), "--");
    }
}
