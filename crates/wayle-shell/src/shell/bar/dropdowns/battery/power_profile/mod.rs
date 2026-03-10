mod messages;
mod watchers;

mod methods;
use std::sync::Arc;

use gtk::prelude::*;
use relm4::{gtk, prelude::*};
use wayle_common::{Property, watchers::WatcherToken};
use wayle_power_profiles::{PowerProfilesService, types::profile::PowerProfile};

pub(crate) use self::messages::PowerProfileInit;
use self::messages::{PowerProfileCmd, PowerProfileInput};
use crate::i18n::t;

pub(crate) struct PowerProfileSection {
    power_profiles: Property<Option<Arc<PowerProfilesService>>>,
    profile_token: WatcherToken,
    active_profile: PowerProfile,

    has_saver: bool,
    has_balanced: bool,
    has_performance: bool,
}

#[relm4::component(pub(crate))]
impl Component for PowerProfileSection {
    type Init = PowerProfileInit;
    type Input = PowerProfileInput;
    type Output = ();
    type CommandOutput = PowerProfileCmd;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            #[name = "section_label"]
            gtk::Label {
                add_css_class: "section-label",
                set_label: &t!("dropdown-battery-power-profile"),
                set_halign: gtk::Align::Start,
            },

            #[name = "profile_segmented"]
            gtk::Box {
                add_css_class: "profile-seg",
                set_homogeneous: true,

                #[name = "saver_btn"]
                gtk::ToggleButton {
                    add_css_class: "profile-seg-btn",
                    set_cursor_from_name: Some("pointer"),
                    set_hexpand: true,
                    #[watch]
                    set_sensitive: model.has_saver,
                    #[watch]
                    #[block_signal(saver_handler)]
                    set_active: model.has_saver && model.active_profile == PowerProfile::PowerSaver,
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.input(PowerProfileInput::ProfileSelected(
                                PowerProfile::PowerSaver,
                            ));
                        }
                    } @saver_handler,

                    gtk::Box {
                        add_css_class: "profile-seg-btn-content",
                        set_halign: gtk::Align::Center,

                        gtk::Image {
                            add_css_class: "profile-seg-icon",
                            set_icon_name: Some("ld-leaf-symbolic"),
                        },

                        gtk::Label {
                            set_label: &t!("dropdown-battery-profile-saver"),
                        },
                    },
                },

                #[name = "balanced_btn"]
                gtk::ToggleButton {
                    add_css_class: "profile-seg-btn",
                    set_cursor_from_name: Some("pointer"),
                    set_hexpand: true,
                    set_group: Some(&saver_btn),
                    #[watch]
                    set_sensitive: model.has_balanced,
                    #[watch]
                    #[block_signal(balanced_handler)]
                    set_active: model.has_balanced && model.active_profile == PowerProfile::Balanced,
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.input(PowerProfileInput::ProfileSelected(
                                PowerProfile::Balanced,
                            ));
                        }
                    } @balanced_handler,

                    gtk::Box {
                        add_css_class: "profile-seg-btn-content",
                        set_halign: gtk::Align::Center,

                        gtk::Image {
                            add_css_class: "profile-seg-icon",
                            set_icon_name: Some("ld-scale-symbolic"),
                        },

                        gtk::Label {
                            set_label: &t!("dropdown-battery-profile-balanced"),
                        },
                    },
                },

                gtk::ToggleButton {
                    add_css_class: "profile-seg-btn",
                    set_cursor_from_name: Some("pointer"),
                    set_hexpand: true,
                    set_group: Some(&saver_btn),
                    #[watch]
                    set_sensitive: model.has_performance,
                    #[watch]
                    #[block_signal(perf_handler)]
                    set_active: model.has_performance && model.active_profile == PowerProfile::Performance,
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.input(PowerProfileInput::ProfileSelected(
                                PowerProfile::Performance,
                            ));
                        }
                    } @perf_handler,

                    gtk::Box {
                        add_css_class: "profile-seg-btn-content",
                        set_halign: gtk::Align::Center,

                        gtk::Image {
                            add_css_class: "profile-seg-icon",
                            set_icon_name: Some("ld-rocket-symbolic"),
                        },

                        gtk::Label {
                            set_label: &t!("dropdown-battery-profile-performance"),
                        },
                    },
                },
            },

            #[name = "profiles_unavailable"]
            gtk::Box {
                add_css_class: "power-profile-not-available",
                #[watch]
                set_visible: !model.has_saver && !model.has_balanced && !model.has_performance,

                gtk::Image {
                    add_css_class: "power-profile-info-icon",
                    set_icon_name: Some("ld-info-symbolic"),
                },

                gtk::Label {
                    add_css_class: "power-profile-info-text",
                    set_label: &t!("dropdown-battery-power-profile-not-available"),
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let current_service = init.power_profiles.get();

        let active_profile = current_service
            .as_ref()
            .map(|service| service.power_profiles.active_profile.get())
            .unwrap_or(PowerProfile::Balanced);

        let available_profiles: Vec<PowerProfile> = current_service
            .as_ref()
            .map(|service| {
                service
                    .power_profiles
                    .profiles
                    .get()
                    .into_iter()
                    .map(|profile| profile.profile)
                    .collect()
            })
            .unwrap_or_default();

        watchers::spawn_availability(&sender, &init.power_profiles);

        let mut profile_token = WatcherToken::new();
        if let Some(service) = &current_service {
            let token = profile_token.reset();
            watchers::spawn_profile_watchers(&sender, service, token);
        }

        let model = Self {
            power_profiles: init.power_profiles,
            profile_token,
            active_profile,
            has_saver: available_profiles.contains(&PowerProfile::PowerSaver),
            has_balanced: available_profiles.contains(&PowerProfile::Balanced),
            has_performance: available_profiles.contains(&PowerProfile::Performance),
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            PowerProfileInput::ProfileSelected(profile) => {
                self.select_profile(profile, &sender);
            }
        }
    }

    fn update_cmd(
        &mut self,
        msg: Self::CommandOutput,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            PowerProfileCmd::ProfileChanged(profile) => {
                self.active_profile = profile;
            }

            PowerProfileCmd::AvailableProfilesChanged(profiles) => {
                self.has_saver = profiles.contains(&PowerProfile::PowerSaver);
                self.has_balanced = profiles.contains(&PowerProfile::Balanced);
                self.has_performance = profiles.contains(&PowerProfile::Performance);
            }

            PowerProfileCmd::ServiceAvailable(service) => {
                self.apply_service(&sender, &service);
            }

            PowerProfileCmd::ServiceUnavailable => {
                self.profile_token = WatcherToken::new();
                self.active_profile = PowerProfile::Balanced;
                self.has_saver = false;
                self.has_balanced = false;
                self.has_performance = false;
            }
        }
    }
}
