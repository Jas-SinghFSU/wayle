//! Glob pattern matching utilities.

use wildcard::Wildcard;

/// Checks if `text` matches a glob `pattern`.
///
/// Patterns support `*` (zero or more characters) and `?` (exactly one character).
///
/// # Examples
///
/// ```
/// use wayle_common::glob::matches;
///
/// assert!(matches("*spotify*", "org.mpris.MediaPlayer2.spotify"));
/// assert!(matches("firefox*", "firefox.instance12345"));
/// assert!(!matches("*chrome*", "firefox"));
/// ```
pub fn matches(pattern: &str, text: &str) -> bool {
    Wildcard::new(pattern.as_bytes())
        .ok()
        .map(|w| w.is_match(text.as_bytes()))
        .unwrap_or(false)
}

/// Finds the first pattern that matches `text` and returns the associated value.
///
/// Useful for mapping player names to icons via pattern-value pairs.
pub fn find_match<'a, V>(
    patterns: impl IntoIterator<Item = (&'a str, V)>,
    text: &str,
) -> Option<V> {
    patterns.into_iter().find_map(|(pattern, value)| {
        if matches(pattern, text) {
            Some(value)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wildcard_contains() {
        assert!(matches("*spotify*", "org.mpris.MediaPlayer2.spotify"));
        assert!(matches("*spotify*", "spotify"));
        assert!(matches("*spotify*", "spotify.instance1"));
    }

    #[test]
    fn wildcard_prefix() {
        assert!(matches("firefox*", "firefox"));
        assert!(matches("firefox*", "firefox.instance12345"));
        assert!(!matches("firefox*", "org.firefox"));
    }

    #[test]
    fn wildcard_suffix() {
        assert!(matches("*.vlc", "org.videolan.vlc"));
        assert!(!matches("*.vlc", "vlc.player"));
    }

    #[test]
    fn exact_match() {
        assert!(matches("vlc", "vlc"));
        assert!(!matches("vlc", "vlc2"));
    }

    #[test]
    fn find_match_returns_first() {
        let mappings = [
            ("*spotify*", "spotify-icon"),
            ("*firefox*", "firefox-icon"),
            ("*", "default-icon"),
        ];

        assert_eq!(
            find_match(mappings.iter().map(|(p, v)| (*p, *v)), "spotify"),
            Some("spotify-icon")
        );
    }

    #[test]
    fn find_match_none() {
        let mappings = [("*spotify*", "spotify-icon")];

        assert_eq!(
            find_match(mappings.iter().map(|(p, v)| (*p, *v)), "firefox"),
            None
        );
    }
}
