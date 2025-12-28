//! Popover components for dropdown menus and selection lists.
#![allow(missing_docs)]

mod header;
mod item;

pub use header::PopoverHeader;
pub use item::PopoverItem;

use gtk4::prelude::PopoverExt;
use gtk4::prelude::WidgetExt;
use relm4::WidgetTemplate;
use relm4::gtk;

/// Styled popover container for dropdown content.
#[relm4::widget_template(pub)]
impl WidgetTemplate for Popover {
    view! {
        gtk::Popover {
            set_css_classes: &["popover"],
            set_has_arrow: false,
        }
    }
}
