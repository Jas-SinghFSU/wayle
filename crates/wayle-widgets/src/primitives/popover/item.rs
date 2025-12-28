use gtk::prelude::*;
use relm4::factory::{FactoryComponent, FactorySender};
use relm4::prelude::*;

/// Data model for a popover list item.
pub struct PopoverItem {
    /// Leading icon name.
    pub icon: Option<String>,
    /// Primary label text.
    pub label: String,
    /// Secondary description text.
    pub subtitle: Option<String>,
    /// Trailing icon shown when active.
    pub active_icon: Option<String>,
    /// Whether this item is currently selected.
    pub is_active: bool,
}

#[relm4::factory(pub)]
impl FactoryComponent for PopoverItem {
    type Init = PopoverItem;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        gtk::ListBoxRow {
            add_css_class: "popover-item",

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,

                gtk::Box {
                    add_css_class: "icon",
                    set_valign: gtk::Align::Center,
                    set_halign: gtk::Align::Start,
                    #[watch]
                    set_visible: self.icon.is_some(),

                    gtk::Image {
                        set_icon_name: self.icon.as_deref(),
                    },
                },

                gtk::Box {
                    add_css_class: "labels",
                    set_orientation: gtk::Orientation::Vertical,
                    set_valign: gtk::Align::Center,
                    set_halign: gtk::Align::Start,
                    set_hexpand: true,

                    gtk::Label {
                        add_css_class: "label",
                        set_halign: gtk::Align::Start,
                        set_ellipsize: gtk::pango::EllipsizeMode::End,
                        #[watch]
                        set_max_width_chars: if self.icon.is_some() { 30 } else { 35 },
                        #[watch]
                        set_label: &self.label,
                    },

                    gtk::Label {
                        add_css_class: "subtitle",
                        set_halign: gtk::Align::Start,
                        set_ellipsize: gtk::pango::EllipsizeMode::End,
                        set_max_width_chars: 30,
                        #[watch]
                        set_label: &self.subtitle.as_deref().unwrap_or(""),
                        #[watch]
                        set_visible: self.subtitle.is_some(),
                    }
                },

                gtk::Image {
                    add_css_class: "active-icon",
                    set_valign: gtk::Align::Center,
                    set_halign: gtk::Align::End,
                    set_icon_name: self.active_icon.as_deref(),
                    #[watch]
                    set_visible: self.is_active && self.active_icon.is_some(),
                }
            }
        }
    }

    fn init_model(init: Self::Init, _index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        init
    }
}
