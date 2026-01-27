mod helpers;
mod messages;
mod watchers;

use relm4::prelude::*;
use wayle_common::{ConfigProperty, WatcherToken, process, services};
use wayle_config::{
    ConfigService,
    schemas::{modules::NetworkConfig, styling::CssToken},
};
use wayle_network::{NetworkService, types::connectivity::ConnectionType};
use wayle_widgets::prelude::{
    BarButton, BarButtonBehavior, BarButtonColors, BarButtonInit, BarButtonInput, BarButtonOutput,
};

use self::helpers::{WifiContext, WiredContext, wifi_icon, wifi_label, wired_icon, wired_label};
pub(crate) use self::messages::{NetworkCmd, NetworkInit, NetworkMsg};

pub(crate) struct NetworkModule {
    bar_button: Controller<BarButton>,
    wifi_watcher: WatcherToken,
    wired_watcher: WatcherToken,
}

#[relm4::component(pub(crate))]
impl Component for NetworkModule {
    type Init = NetworkInit;
    type Input = NetworkMsg;
    type Output = ();
    type CommandOutput = NetworkCmd;

    view! {
        gtk::Box {
            #[local_ref]
            bar_button -> gtk::MenuButton {},
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let config_service = services::get::<ConfigService>();
        let config = config_service.config();
        let network_config = &config.modules.network;

        let (initial_icon, initial_label) = Self::compute_display(network_config);

        let bar_button = BarButton::builder()
            .launch(BarButtonInit {
                icon: initial_icon,
                label: initial_label,
                tooltip: None,
                colors: BarButtonColors {
                    icon_color: network_config.icon_color.clone(),
                    label_color: network_config.label_color.clone(),
                    icon_background: network_config.icon_bg_color.clone(),
                    button_background: network_config.button_bg_color.clone(),
                    border_color: network_config.border_color.clone(),
                    auto_icon_color: CssToken::Accent,
                },
                behavior: BarButtonBehavior {
                    label_max_chars: network_config.label_max_length.clone(),
                    show_icon: network_config.icon_show.clone(),
                    show_label: network_config.label_show.clone(),
                    show_border: network_config.border_show.clone(),
                    visible: ConfigProperty::new(true),
                },
                settings: init.settings,
            })
            .forward(sender.input_sender(), |output| match output {
                BarButtonOutput::LeftClick => NetworkMsg::LeftClick,
                BarButtonOutput::RightClick => NetworkMsg::RightClick,
                BarButtonOutput::MiddleClick => NetworkMsg::MiddleClick,
                BarButtonOutput::ScrollUp => NetworkMsg::ScrollUp,
                BarButtonOutput::ScrollDown => NetworkMsg::ScrollDown,
            });

        watchers::spawn_watchers(&sender, network_config);

        let mut wifi_watcher = WatcherToken::new();
        let mut wired_watcher = WatcherToken::new();

        watchers::spawn_wifi_watchers(&sender, wifi_watcher.reset());
        watchers::spawn_wired_watchers(&sender, wired_watcher.reset());

        let model = Self {
            bar_button,
            wifi_watcher,
            wired_watcher,
        };
        let bar_button = model.bar_button.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        let config_service = services::get::<ConfigService>();
        let config = &config_service.config().modules.network;

        let cmd = match msg {
            NetworkMsg::LeftClick => config.left_click.get(),
            NetworkMsg::RightClick => config.right_click.get(),
            NetworkMsg::MiddleClick => config.middle_click.get(),
            NetworkMsg::ScrollUp => config.scroll_up.get(),
            NetworkMsg::ScrollDown => config.scroll_down.get(),
        };

        process::run_if_set(&cmd);
    }

    fn update_cmd(&mut self, msg: NetworkCmd, sender: ComponentSender<Self>, _root: &Self::Root) {
        let config_service = services::get::<ConfigService>();
        let network_config = &config_service.config().modules.network;

        match msg {
            NetworkCmd::StateChanged | NetworkCmd::IconConfigChanged => {
                let (icon, label) = Self::compute_display(network_config);
                self.bar_button.emit(BarButtonInput::SetIcon(icon));
                self.bar_button.emit(BarButtonInput::SetLabel(label));
            }
            NetworkCmd::WifiDeviceChanged => {
                let token = self.wifi_watcher.reset();
                watchers::spawn_wifi_watchers(&sender, token);

                let (icon, label) = Self::compute_display(network_config);
                self.bar_button.emit(BarButtonInput::SetIcon(icon));
                self.bar_button.emit(BarButtonInput::SetLabel(label));
            }
            NetworkCmd::WiredDeviceChanged => {
                let token = self.wired_watcher.reset();
                watchers::spawn_wired_watchers(&sender, token);

                let (icon, label) = Self::compute_display(network_config);
                self.bar_button.emit(BarButtonInput::SetIcon(icon));
                self.bar_button.emit(BarButtonInput::SetLabel(label));
            }
        }
    }
}

impl NetworkModule {
    fn compute_display(config: &NetworkConfig) -> (String, String) {
        let network_service = services::get::<NetworkService>();
        let primary = network_service.primary.get();

        match primary {
            ConnectionType::Wifi => {
                if let Some(wifi) = network_service.wifi.get() {
                    let ssid = wifi.ssid.get();
                    let ctx = WifiContext {
                        enabled: wifi.enabled.get(),
                        connectivity: wifi.connectivity.get(),
                        strength: wifi.strength.get(),
                        ssid: ssid.as_deref(),
                    };
                    (wifi_icon(config, &ctx), wifi_label(&ctx))
                } else {
                    (
                        config.wifi_offline_icon.get().clone(),
                        String::from("No WiFi"),
                    )
                }
            }
            ConnectionType::Wired => {
                if let Some(wired) = network_service.wired.get() {
                    let ctx = WiredContext {
                        connectivity: wired.connectivity.get(),
                    };
                    (wired_icon(config, &ctx), wired_label(&ctx))
                } else {
                    (
                        config.wired_disconnected_icon.get().clone(),
                        String::from("No Ethernet"),
                    )
                }
            }
            ConnectionType::Unknown => {
                if let Some(wifi) = network_service.wifi.get() {
                    let ssid = wifi.ssid.get();
                    let ctx = WifiContext {
                        enabled: wifi.enabled.get(),
                        connectivity: wifi.connectivity.get(),
                        strength: wifi.strength.get(),
                        ssid: ssid.as_deref(),
                    };
                    (wifi_icon(config, &ctx), wifi_label(&ctx))
                } else if let Some(wired) = network_service.wired.get() {
                    let ctx = WiredContext {
                        connectivity: wired.connectivity.get(),
                    };
                    (wired_icon(config, &ctx), wired_label(&ctx))
                } else {
                    (
                        config.wifi_offline_icon.get().clone(),
                        String::from("Offline"),
                    )
                }
            }
        }
    }
}
