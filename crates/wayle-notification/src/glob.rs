//! Glob pattern matching utilities.

use wildcard::Wildcard;

/// Returns true if the text matches the glob pattern.
pub fn matches(pattern: &str, text: &str) -> bool {
    Wildcard::new(pattern.as_bytes())
        .ok()
        .map(|w| w.is_match(text.as_bytes()))
        .unwrap_or(false)
}
