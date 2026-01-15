//! Card container widget template.
#![allow(missing_docs)]

use gtk4::prelude::WidgetExt;
use relm4::{WidgetTemplate, gtk};

/// CSS class constants for Card variants.
///
/// Applied via `add_css_class` for optional styling:
/// ```ignore
/// #[template]
/// Card {
///     add_css_class: CardClass::BORDERED,
///     add_css_class: CardClass::COMPACT,
/// }
/// ```
pub struct CardClass;

impl CardClass {
    const BASE: &'static str = "card";

    /// Adds a subtle border.
    pub const BORDERED: &'static str = "bordered";
    /// Reduces internal padding.
    pub const COMPACT: &'static str = "compact";
    /// Increases internal padding.
    pub const SPACIOUS: &'static str = "spacious";
    /// Adds drop shadow.
    pub const SHADOWED: &'static str = "shadowed";
}

/// Elevated container for grouping related content.
#[relm4::widget_template(pub)]
impl WidgetTemplate for Card {
    view! {
        gtk::Box {
            set_css_classes: &[CardClass::BASE],
        }
    }
}
