use gdk4::{
    gio::prelude::ListModelExt,
    glib::object::Cast,
    prelude::{DisplayExt, MonitorExt},
};
use relm4::gtk::gdk;

#[allow(clippy::expect_used)]
pub fn get_current_monitors() -> Vec<(String, gdk::Monitor)> {
    let display = gdk::Display::default().expect("No GDK display found...");
    let current_monitors: Vec<(String, gdk::Monitor)> = (0..display.monitors().n_items())
        .filter_map(|i| display.monitors().item(i))
        .filter_map(|obj| obj.downcast::<gdk::Monitor>().ok())
        .filter_map(|m| m.connector().map(|c| (c.to_string(), m)))
        .collect();

    current_monitors
}
