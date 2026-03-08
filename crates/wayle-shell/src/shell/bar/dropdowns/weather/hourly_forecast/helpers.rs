use chrono::{NaiveDateTime, Timelike};
use wayle_config::schemas::modules::TimeFormat;

pub(super) fn hourly_time_label(time: NaiveDateTime, format: TimeFormat) -> String {
    match format {
        TimeFormat::TwelveHour => {
            let hour = time.format("%-I").to_string();
            let period = time.format("%p").to_string();
            format!("{hour}{period}")
        }
        TimeFormat::TwentyFourHour => {
            format!("{:02}:{:02}", time.hour(), time.minute())
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn twelve_hour_format() {
        let time = NaiveDate::from_ymd_opt(2024, 1, 15)
            .unwrap()
            .and_hms_opt(14, 0, 0)
            .unwrap();
        let result = hourly_time_label(time, TimeFormat::TwelveHour);
        assert_eq!(result, "2PM");
    }

    #[test]
    fn twenty_four_hour_format() {
        let time = NaiveDate::from_ymd_opt(2024, 1, 15)
            .unwrap()
            .and_hms_opt(14, 0, 0)
            .unwrap();
        let result = hourly_time_label(time, TimeFormat::TwentyFourHour);
        assert_eq!(result, "14:00");
    }
}
