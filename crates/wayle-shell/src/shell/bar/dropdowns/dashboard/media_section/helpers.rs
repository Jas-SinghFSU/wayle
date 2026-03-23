use std::time::Duration;

const SECONDS_PER_MINUTE: u64 = 60;

pub(super) fn format_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let minutes = total_secs / SECONDS_PER_MINUTE;
    let seconds = total_secs % SECONDS_PER_MINUTE;
    format!("{minutes}:{seconds:02}")
}
