pub(crate) mod messages;
mod methods;

use std::sync::Arc;

use gtk::prelude::*;
use relm4::{gtk, prelude::*};
use wayle_notification::core::notification::Notification;

use self::messages::{NotificationItemInit, NotificationItemInput, NotificationItemOutput};
use crate::shell::notification_popup::helpers::{ResolvedIcon, relative_time, urgency_css_class};

pub(crate) struct NotificationItem {
    pub(crate) notification: Arc<Notification>,

    resolved_icon: ResolvedIcon,
    time_label: String,
}

#[relm4::factory(pub(crate))]
impl FactoryComponent for NotificationItem {
    type Init = NotificationItemInit;
    type Input = NotificationItemInput;
    type Output = NotificationItemOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        #[root]
        gtk::Box {
            add_css_class: "notification-dropdown-item",
            add_css_class: urgency_css_class(self.notification.urgency.get()),
            set_orientation: gtk::Orientation::Vertical,

            #[name = "main_row"]
            gtk::Box {
                add_css_class: "notification-dropdown-item-main",

                #[name = "icon_container"]
                gtk::Box {
                    add_css_class: "notification-dropdown-item-icon",
                    set_valign: gtk::Align::Start,

                    #[name = "icon"]
                    gtk::Image {
                        add_css_class: "notification-dropdown-item-icon-img",
                        set_halign: gtk::Align::Center,
                        set_valign: gtk::Align::Center,
                    },
                },

                gtk::Box {
                    add_css_class: "notification-dropdown-item-content",
                    set_orientation: gtk::Orientation::Vertical,
                    set_hexpand: true,

                    gtk::Box {
                        add_css_class: "notification-dropdown-item-header",

                        gtk::Label {
                            add_css_class: "notification-dropdown-item-title",
                            set_hexpand: true,
                            set_halign: gtk::Align::Start,
                            set_ellipsize: gtk::pango::EllipsizeMode::End,
                            set_label: &self.notification.summary.get(),
                        },

                        gtk::Label {
                            add_css_class: "notification-dropdown-item-time",
                            #[watch]
                            set_label: &self.time_label,
                        },

                        #[name = "dismiss_btn"]
                        gtk::Button {
                            set_css_classes: &["ghost-icon", "notification-dropdown-item-dismiss"],
                            set_icon_name: "ld-x-symbolic",
                            set_cursor_from_name: Some("pointer"),
                        },
                    },

                    gtk::Label {
                        add_css_class: "notification-dropdown-item-body",
                        set_halign: gtk::Align::Start,
                        set_use_markup: true,
                        set_ellipsize: gtk::pango::EllipsizeMode::End,
                        set_lines: 2,
                        set_wrap: true,
                        set_wrap_mode: gtk::pango::WrapMode::WordChar,
                        set_label: &self.notification.body.get().as_deref().unwrap_or(""),
                        set_visible: self.notification.body.get().is_some(),
                    },
                },
            },

            #[name = "actions_box"]
            gtk::Box {
                add_css_class: "notification-dropdown-item-actions",
                set_orientation: gtk::Orientation::Vertical,
            },
        }
    }

    fn init_model(init: Self::Init, _index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        let time_label = Self::time_to_string(relative_time(&init.notification.timestamp.get()));

        Self {
            notification: init.notification,
            resolved_icon: init.resolved_icon,
            time_label,
        }
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let widgets = view_output!();

        self.apply_icon(&widgets.icon, &widgets.icon_container);
        self.build_action_buttons(&widgets.actions_box);

        let id = self.notification.id;
        let output_sender = sender.output_sender().clone();

        widgets.dismiss_btn.connect_clicked(move |_| {
            output_sender.emit(NotificationItemOutput::Dismissed(id));
        });

        self.setup_default_action(&widgets.main_row);

        widgets
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        match msg {
            NotificationItemInput::RefreshTime => {
                self.time_label =
                    Self::time_to_string(relative_time(&self.notification.timestamp.get()));
            }
        }
    }
}
