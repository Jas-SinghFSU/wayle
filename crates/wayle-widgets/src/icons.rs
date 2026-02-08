//! Icon theme utilities with cached IconTheme reference.

use std::cell::OnceCell;

use gtk4::{IconTheme, gdk::Display};

thread_local! {
    static ICON_THEME: OnceCell<IconTheme> = const { OnceCell::new() };
}

#[allow(clippy::expect_used)]
fn with_icon_theme<R>(f: impl FnOnce(&IconTheme) -> R) -> R {
    ICON_THEME.with(|cell| {
        let theme = cell.get_or_init(|| {
            let display = Display::default().expect("display required for icon theme");
            IconTheme::for_display(&display)
        });
        f(theme)
    })
}

/// Checks if an icon exists in the current icon theme.
pub fn icon_exists(name: &str) -> bool {
    with_icon_theme(|theme| theme.has_icon(name))
}
