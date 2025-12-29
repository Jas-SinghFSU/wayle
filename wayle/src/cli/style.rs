//! ANSI styling for CLI help output.
//!
//! Matches clap's default header styling (bold yellow).

/// Formats a section header to match clap's styling.
#[macro_export]
macro_rules! styled_header {
    ($text:expr) => {
        concat!("\x1b[1;33m", $text, "\x1b[0m")
    };
}
