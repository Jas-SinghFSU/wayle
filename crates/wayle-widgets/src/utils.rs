//! GTK utility functions and workarounds.

use gtk4::{
    glib,
    glib::object::IsA,
    prelude::{Cast, GtkWindowExt, WidgetExt},
};
use relm4::gtk;

/// Resets a layer-shell window's cached size so GTK recalculates from content.
///
/// GTK4 windows remember their largest allocated size and refuse to shrink.
/// Setting default size to (1,1) forces GTK to recompute from minimum, then
/// resetting to (0,0) ensures the next poke also triggers a change.
///
/// The exclusive zone is managed separately by the bar's tick callback,
/// so the transient 1px default does not cause compositor flicker.
pub fn force_window_resize(widget: &impl IsA<gtk::Widget>) {
    if let Some(root) = widget.as_ref().root()
        && let Ok(window) = root.downcast::<gtk::Window>()
    {
        glib::idle_add_local_once(move || {
            window.set_default_size(1, 1);
            window.set_default_size(0, 0);
        });
    }
}
