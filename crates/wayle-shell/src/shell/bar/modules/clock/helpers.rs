use gtk4::glib::DateTime;
use tracing::error;

pub(super) fn format_time(format: &str) -> String {
    DateTime::now_local()
        .and_then(|dt| dt.format(format))
        .map(|gstring| gstring.to_string())
        .inspect_err(|e| error!(error = %e, "cannot format time"))
        .unwrap_or_else(|_| String::from("--"))
}
