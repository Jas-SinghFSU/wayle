mod helpers;
mod messages;
mod methods;
mod watchers;

use std::sync::Arc;

use gtk::prelude::*;
use relm4::{gtk, prelude::*};
use wayle_sysinfo::SysinfoService;
use wayle_widgets::{
    WatcherToken,
    primitives::progress_ring::{ProgressRing, ProgressRingInit, Size},
};

use self::messages::SystemStatsCmd;
pub(crate) use self::messages::{SystemStatsInit, SystemStatsInput};
use crate::i18n::t;

pub(crate) struct SystemStatsSection {
    sysinfo: Arc<SysinfoService>,
    active: bool,
    watcher: WatcherToken,
    cpu_ring: Controller<ProgressRing>,
    mem_ring: Controller<ProgressRing>,
    disk_ring: Controller<ProgressRing>,
    temp_ring: Controller<ProgressRing>,
}

#[relm4::component(pub(crate))]
impl Component for SystemStatsSection {
    type Init = SystemStatsInit;
    type Input = SystemStatsInput;
    type Output = ();
    type CommandOutput = SystemStatsCmd;

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
                        set_icon_name: Some("ld-activity-symbolic"),
                    },

                    gtk::Label {
                        set_label: &t!("dropdown-dashboard-system"),
                    },
                },
            },

            #[name = "stats_container"]
            gtk::Box {
                add_css_class: "system-stats-inline",

                #[name = "cpu_stat"]
                gtk::Box {
                    add_css_class: "stat-inline",
                    set_orientation: gtk::Orientation::Vertical,

                    #[local_ref]
                    cpu_ring_widget -> gtk::Overlay {},

                    gtk::Label {
                        add_css_class: "stat-label",
                        set_label: &t!("dropdown-dashboard-cpu"),
                    },
                },

                #[name = "mem_stat"]
                gtk::Box {
                    add_css_class: "stat-inline",
                    set_orientation: gtk::Orientation::Vertical,

                    #[local_ref]
                    mem_ring_widget -> gtk::Overlay {},

                    gtk::Label {
                        add_css_class: "stat-label",
                        set_label: &t!("dropdown-dashboard-ram"),
                    },
                },

                #[name = "disk_stat"]
                gtk::Box {
                    add_css_class: "stat-inline",
                    set_orientation: gtk::Orientation::Vertical,

                    #[local_ref]
                    disk_ring_widget -> gtk::Overlay {},

                    gtk::Label {
                        add_css_class: "stat-label",
                        set_label: &t!("dropdown-dashboard-disk"),
                    },
                },

                #[name = "temp_stat"]
                gtk::Box {
                    add_css_class: "stat-inline",
                    set_orientation: gtk::Orientation::Vertical,

                    #[local_ref]
                    temp_ring_widget -> gtk::Overlay {},

                    gtk::Label {
                        add_css_class: "stat-label",
                        set_label: &t!("dropdown-dashboard-temp"),
                    },
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let cpu_ring = ProgressRing::builder()
            .launch(ProgressRingInit {
                fraction: 0.0,
                size: Size::Lg,
                ..Default::default()
            })
            .detach();

        let mem_ring = ProgressRing::builder()
            .launch(ProgressRingInit {
                fraction: 0.0,
                size: Size::Lg,
                ..Default::default()
            })
            .detach();

        let disk_ring = ProgressRing::builder()
            .launch(ProgressRingInit {
                fraction: 0.0,
                size: Size::Lg,
                ..Default::default()
            })
            .detach();

        let temp_ring = ProgressRing::builder()
            .launch(ProgressRingInit {
                fraction: 0.0,
                size: Size::Lg,
                ..Default::default()
            })
            .detach();

        let cpu_data = init.sysinfo.cpu.get();

        methods::update_usage_ring(&cpu_ring, cpu_data.usage_percent);

        if let Some(celsius) = cpu_data.temperature_celsius {
            methods::update_temp_ring(&temp_ring, celsius);
        }

        methods::update_usage_ring(&mem_ring, init.sysinfo.memory.get().usage_percent);

        let disk_usage = init
            .sysinfo
            .disks
            .get()
            .iter()
            .find(|disk| disk.mount_point.as_os_str() == "/")
            .map_or(0.0, |disk| disk.usage_percent);
        methods::update_usage_ring(&disk_ring, disk_usage);

        let model = Self {
            sysinfo: init.sysinfo,
            active: false,
            watcher: WatcherToken::new(),
            cpu_ring,
            mem_ring,
            disk_ring,
            temp_ring,
        };

        let cpu_ring_widget = model.cpu_ring.widget();
        let mem_ring_widget = model.mem_ring.widget();
        let disk_ring_widget = model.disk_ring.widget();
        let temp_ring_widget = model.temp_ring.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            SystemStatsInput::SetActive(active) => {
                if self.active == active {
                    return;
                }

                self.active = active;
                if active {
                    let token = self.watcher.reset();
                    watchers::spawn(&sender, &self.sysinfo, token);
                } else {
                    self.watcher = WatcherToken::new();
                }
            }
        }
    }

    fn update_cmd(
        &mut self,
        msg: SystemStatsCmd,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            SystemStatsCmd::CpuChanged { usage, temp } => {
                methods::update_usage_ring(&self.cpu_ring, usage);
                if let Some(celsius) = temp {
                    methods::update_temp_ring(&self.temp_ring, celsius);
                }
            }

            SystemStatsCmd::MemoryChanged { usage } => {
                methods::update_usage_ring(&self.mem_ring, usage);
            }

            SystemStatsCmd::DiskChanged { usage } => {
                methods::update_usage_ring(&self.disk_ring, usage);
            }
        }
    }
}
