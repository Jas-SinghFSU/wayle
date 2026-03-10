use relm4::gtk;

use super::SeparatorModule;

impl SeparatorModule {
    /// Returns the separator orientation based on bar orientation.
    pub(super) fn orientation_for_vertical(is_vertical: bool) -> gtk::Orientation {
        if is_vertical {
            gtk::Orientation::Horizontal
        } else {
            gtk::Orientation::Vertical
        }
    }
}
