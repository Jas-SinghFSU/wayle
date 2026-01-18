//! Internationalization for Wayle using Mozilla Fluent.
//!
//! ```ignore
//! use wayle_i18n::t;
//!
//! let text = t!("app-name");
//! let greeting = t!("welcome-user", user = "Alice");
//! ```

use std::sync::OnceLock;

use i18n_embed::{
    DesktopLanguageRequester, LanguageLoader,
    fluent::{FluentLanguageLoader, fluent_language_loader},
};
pub use i18n_embed_fl::fl;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "locales/"]
struct Localizations;

static LOADER: OnceLock<FluentLanguageLoader> = OnceLock::new();

/// Returns the language loader, auto-detecting system locale on first access.
///
/// # Panics
///
/// Panics if embedded FTL resources fail to load.
#[allow(clippy::expect_used)]
pub fn loader() -> &'static FluentLanguageLoader {
    LOADER.get_or_init(|| {
        let loader = fluent_language_loader!();
        loader
            .load_fallback_language(&Localizations)
            .expect("embedded FTL resources are valid");

        let requested = DesktopLanguageRequester::requested_languages();
        let _ = i18n_embed::select(&loader, &Localizations, &requested);

        loader
    })
}

/// Looks up a translated message by key.
#[macro_export]
macro_rules! t {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::loader(), $message_id)
    }};
    ($message_id:literal, $($args:tt)*) => {{
        i18n_embed_fl::fl!($crate::loader(), $message_id, $($args)*)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_from_both_files_work() {
        let _ = t!("app-name");
        let _ = t!("settings-bar-scale");
    }
}
