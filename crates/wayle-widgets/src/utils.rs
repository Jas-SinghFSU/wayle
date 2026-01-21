//! GTK utility functions and workarounds.

use gtk4::glib;
use gtk4::glib::object::IsA;
use gtk4::prelude::{Cast, GtkWindowExt, WidgetExt};
use relm4::gtk;

/// Forces a layer-shell window to recalculate its size.
///
/// GTK4 windows don't automatically shrink when child content shrinks.
/// For layer-shell surfaces, this is the only reliable way to trigger
/// a size recalculation that respects anchor constraints.
pub fn force_window_resize(widget: &impl IsA<gtk::Widget>) {
    if let Some(root) = widget.as_ref().root()
        && let Ok(window) = root.downcast::<gtk::Window>()
    {
        glib::idle_add_local_once(move || {
            window.set_default_size(1, 1);
        });
    }
}
