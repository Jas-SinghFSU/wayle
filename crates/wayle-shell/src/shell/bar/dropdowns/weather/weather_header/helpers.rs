use chrono::{DateTime, Utc};

pub(super) fn updated_ago_minutes(updated_at: DateTime<Utc>) -> i64 {
    let elapsed = Utc::now() - updated_at;
    elapsed.num_minutes().max(0)
}

pub(super) fn location_display(city: &str, region: Option<&str>, country: &str) -> String {
    match region {
        Some(region) if !region.is_empty() => format!("{city}, {region}"),
        _ => format!("{city}, {country}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn location_with_region() {
        let result = location_display("San Francisco", Some("California"), "US");
        assert_eq!(result, "San Francisco, California");
    }

    #[test]
    fn location_without_region() {
        let result = location_display("London", None, "United Kingdom");
        assert_eq!(result, "London, United Kingdom");
    }

    #[test]
    fn location_with_empty_region() {
        let result = location_display("Tokyo", Some(""), "Japan");
        assert_eq!(result, "Tokyo, Japan");
    }

    #[test]
    fn updated_ago_is_non_negative() {
        let future = Utc::now() + chrono::Duration::hours(1);
        assert_eq!(updated_ago_minutes(future), 0);
    }
}
