//! Internationalization for wayle-shell runtime labels.

use std::sync::OnceLock;

use i18n_embed::{
    DesktopLanguageRequester, LanguageLoader,
    fluent::{FluentLanguageLoader, fluent_language_loader},
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "locales/"]
struct Localizations;

static LOADER: OnceLock<FluentLanguageLoader> = OnceLock::new();

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

macro_rules! t {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::i18n::loader(), $message_id)
    }};
    ($message_id:literal, $($args:tt)*) => {{
        i18n_embed_fl::fl!($crate::i18n::loader(), $message_id, $($args)*)
    }};
}

pub(crate) use t;
