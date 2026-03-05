use chrono::{NaiveTime, Timelike};
use wayle_config::schemas::modules::TimeFormat;

pub(super) fn format_time(time: NaiveTime, format: TimeFormat) -> String {
    match format {
        TimeFormat::TwelveHour => format_time_12h(time),
        TimeFormat::TwentyFourHour => format_time_24h(time),
    }
}

fn format_time_12h(time: NaiveTime) -> String {
    let hour = time.hour();
    let minute = time.minute();
    let period = if hour < 12 { "AM" } else { "PM" };
    let display_hour = match hour {
        0 => 12,
        13..=23 => hour - 12,
        other => other,
    };
    format!("{display_hour}:{minute:02} {period}")
}

fn format_time_24h(time: NaiveTime) -> String {
    let hour = time.hour();
    let minute = time.minute();
    format!("{hour:02}:{minute:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twelve_hour_morning() {
        let time = NaiveTime::from_hms_opt(6, 32, 0).unwrap();
        assert_eq!(format_time(time, TimeFormat::TwelveHour), "6:32 AM");
    }

    #[test]
    fn twelve_hour_afternoon() {
        let time = NaiveTime::from_hms_opt(19, 48, 0).unwrap();
        assert_eq!(format_time(time, TimeFormat::TwelveHour), "7:48 PM");
    }

    #[test]
    fn twelve_hour_noon() {
        let time = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        assert_eq!(format_time(time, TimeFormat::TwelveHour), "12:00 PM");
    }

    #[test]
    fn twelve_hour_midnight() {
        let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        assert_eq!(format_time(time, TimeFormat::TwelveHour), "12:00 AM");
    }

    #[test]
    fn twelve_hour_just_before_noon() {
        let time = NaiveTime::from_hms_opt(11, 59, 0).unwrap();
        assert_eq!(format_time(time, TimeFormat::TwelveHour), "11:59 AM");
    }

    #[test]
    fn twenty_four_hour_morning() {
        let time = NaiveTime::from_hms_opt(6, 32, 0).unwrap();
        assert_eq!(format_time(time, TimeFormat::TwentyFourHour), "06:32");
    }

    #[test]
    fn twenty_four_hour_afternoon() {
        let time = NaiveTime::from_hms_opt(19, 48, 0).unwrap();
        assert_eq!(format_time(time, TimeFormat::TwentyFourHour), "19:48");
    }

    #[test]
    fn twenty_four_hour_midnight() {
        let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        assert_eq!(format_time(time, TimeFormat::TwentyFourHour), "00:00");
    }
}
