mod messages;
mod methods;
mod watchers;

use std::sync::Arc;

use gtk::prelude::*;
use relm4::{gtk, prelude::*};
use wayle_battery::types::DeviceState;
use wayle_power_profiles::{PowerProfilesService, types::profile::PowerProfile};
use wayle_widgets::{
    WatcherToken,
    primitives::progress_bar::{ProgressBar, ProgressBarClass},
};

pub(crate) use self::messages::BatterySectionInit;
use self::{
    messages::BatterySectionCmd,
    methods::{BATTERY_CRITICAL, BATTERY_WARNING},
};
use crate::i18n::t;

const PERCENTAGE_DIVISOR: f64 = 100.0;

pub(crate) struct BatterySection {
    power_profiles: Option<Arc<PowerProfilesService>>,
    profile_watcher: WatcherToken,

    percentage: f64,
    state: DeviceState,
    time_remaining_secs: i64,
    is_warning: bool,
    is_critical: bool,

    power_profile: PowerProfile,
    has_power_profiles: bool,
}

#[relm4::component(pub(crate))]
impl Component for BatterySection {
    type Init = BatterySectionInit;
    type Input = ();
    type Output = ();
    type CommandOutput = BatterySectionCmd;

    view! {
        #[root]
        gtk::Box {
            set_css_classes: &["card", "dashboard-card"],
            set_orientation: gtk::Orientation::Vertical,

            #[name = "header"]
            gtk::Box {
                add_css_class: "card-header",

                #[name = "card_title"]
                gtk::Box {
                    add_css_class: "card-title",

                    gtk::Image {
                        set_icon_name: Some("ld-battery-full-symbolic"),
                    },

                    gtk::Label {
                        set_label: &t!("dropdown-dashboard-battery"),
                    },
                },
            },

            #[name = "battery_status"]
            gtk::Box {
                add_css_class: "battery-status",

                #[name = "battery_icon"]
                gtk::Box {
                    add_css_class: "battery-icon",
                    #[watch]
                    set_class_active: ("warning", model.is_warning),
                    #[watch]
                    set_class_active: ("critical", model.is_critical),

                    gtk::Image {
                        #[watch]
                        set_icon_name: Some(model.battery_icon()),
                    },
                },

                #[name = "percentage_label"]
                gtk::Label {
                    add_css_class: "battery-percent",
                    #[watch]
                    set_class_active: ("warning", model.is_warning),
                    #[watch]
                    set_class_active: ("critical", model.is_critical),
                    #[watch]
                    set_label: &format!("{:.0}%", model.percentage),
                },
            },

            #[template]
            #[name = "gauge"]
            ProgressBar {
                add_css_class: ProgressBarClass::SMALL,
                #[watch]
                set_class_active: (ProgressBarClass::SUCCESS, !model.is_warning && !model.is_critical),
                #[watch]
                set_class_active: (ProgressBarClass::WARNING, model.is_warning),
                #[watch]
                set_class_active: (ProgressBarClass::ERROR, model.is_critical),
                #[watch]
                set_fraction: model.percentage / PERCENTAGE_DIVISOR,
            },

            #[name = "time_remaining"]
            gtk::Label {
                add_css_class: "battery-detail",
                set_halign: gtk::Align::Start,
                #[watch]
                set_class_active: ("warning", model.is_warning),
                #[watch]
                set_class_active: ("critical", model.is_critical),
                #[watch]
                set_visible: model.time_remaining_secs > 0,
                #[watch]
                set_label: &model.time_remaining_label(),
            },

            #[name = "profile_row"]
            gtk::Box {
                add_css_class: "battery-profile",
                set_halign: gtk::Align::Start,
                #[watch]
                set_visible: model.has_power_profiles,

                #[name = "profile_icon"]
                gtk::Image {
                    add_css_class: "battery-profile-icon",
                    #[watch]
                    set_icon_name: Some(model.power_profile_icon()),
                },

                #[name = "profile_label"]
                gtk::Label {
                    add_css_class: "battery-profile-label",
                    #[watch]
                    set_label: &model.power_profile_label(),
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
        let has_power_profiles = current_service.is_some();

        let power_profile = current_service
            .as_ref()
            .map(|service| service.power_profiles.active_profile.get())
            .unwrap_or(PowerProfile::Balanced);

        watchers::spawn_power_profiles_watcher(&sender, &init.power_profiles);

        let mut profile_watcher = WatcherToken::new();

        if let Some(service) = &current_service {
            let token = profile_watcher.reset();
            watchers::spawn_active_profile_watcher(&sender, service, token);
        }

        let percentage = init
            .battery
            .as_ref()
            .map(|battery| {
                watchers::spawn(&sender, battery);
                battery.device.percentage.get()
            })
            .unwrap_or(0.0);

        let model = Self {
            power_profiles: current_service,
            profile_watcher,

            percentage,
            state: DeviceState::Unknown,
            time_remaining_secs: 0,

            is_warning: percentage <= BATTERY_WARNING && percentage > BATTERY_CRITICAL,
            is_critical: percentage <= BATTERY_CRITICAL,

            power_profile,
            has_power_profiles,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update_cmd(
        &mut self,
        msg: BatterySectionCmd,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            BatterySectionCmd::StateChanged {
                percentage,
                state,
                time_remaining_secs,
            } => {
                self.percentage = percentage;
                self.state = state;
                self.time_remaining_secs = time_remaining_secs;

                self.is_warning = percentage <= BATTERY_WARNING && percentage > BATTERY_CRITICAL;
                self.is_critical = percentage <= BATTERY_CRITICAL;
            }

            BatterySectionCmd::PowerProfileChanged(profile) => {
                self.power_profile = profile;
            }

            BatterySectionCmd::PowerProfilesAvailable(service) => {
                self.has_power_profiles = true;
                self.power_profile = service.power_profiles.active_profile.get();

                let token = self.profile_watcher.reset();
                watchers::spawn_active_profile_watcher(&sender, &service, token);

                self.power_profiles = Some(service);
            }

            BatterySectionCmd::PowerProfilesUnavailable => {
                self.profile_watcher = WatcherToken::new();
                self.has_power_profiles = false;
                self.power_profiles = None;
            }
        }
    }
}
