use gtk::{pango, prelude::*};
use relm4::{gtk, prelude::*};
use wayle_media::types::PlayerId;

pub(super) struct SourceItemInit {
    pub identity: String,
    pub player_id: PlayerId,
    pub icon_name: String,
    pub active: bool,
}

pub(super) struct SourceItem {
    identity: String,
    player_id: PlayerId,
    icon_name: String,
    active: bool,
}

impl SourceItem {
    pub(super) fn player_id(&self) -> PlayerId {
        self.player_id.clone()
    }
}

#[relm4::factory(pub(super))]
impl FactoryComponent for SourceItem {
    type Init = SourceItemInit;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        #[root]
        gtk::ListBoxRow {
            add_css_class: "media-source-option",
            set_activatable: true,
            set_cursor_from_name: Some("pointer"),
            #[watch]
            set_css_classes: if self.active {
                &["media-source-option", "selected"]
            } else {
                &["media-source-option"]
            },

            gtk::Box {
                add_css_class: "media-source-option-content",

                gtk::CenterBox {
                    add_css_class: "media-source-option-icon",
                    set_valign: gtk::Align::Center,
                    #[wrap(Some)]
                    set_center_widget = &gtk::Image {
                        set_icon_name: Some(&self.icon_name),
                    },
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_hexpand: true,
                    set_halign: gtk::Align::Start,
                    set_valign: gtk::Align::Center,

                    gtk::Label {
                        add_css_class: "media-source-option-name",
                        set_halign: gtk::Align::Start,
                        set_ellipsize: pango::EllipsizeMode::End,
                        #[watch]
                        set_label: &self.identity,
                    },
                },

                gtk::Image {
                    add_css_class: "media-source-option-check",
                    set_icon_name: Some("tb-check-symbolic"),
                    #[watch]
                    set_visible: self.active,
                },
            },
        }
    }

    fn init_model(init: Self::Init, _index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        Self {
            identity: init.identity,
            player_id: init.player_id,
            icon_name: init.icon_name,
            active: init.active,
        }
    }
}
