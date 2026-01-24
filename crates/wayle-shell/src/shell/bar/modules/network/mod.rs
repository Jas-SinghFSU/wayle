mod helpers;
mod messages;

use relm4::prelude::*;
use tracing::error;
use wayle_common::{ConfigProperty, process::spawn_shell_quiet, services, watch};
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

        Self::spawn_watchers(&sender, network_config);

        let model = Self { bar_button };
        let bar_button = model.bar_button.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        let config_service = services::get::<ConfigService>();
        let network_config = &config_service.config().modules.network;

        let cmd = match msg {
            NetworkMsg::LeftClick => network_config.left_click.get().clone(),
            NetworkMsg::RightClick => network_config.right_click.get().clone(),
            NetworkMsg::MiddleClick => network_config.middle_click.get().clone(),
            NetworkMsg::ScrollUp => network_config.scroll_up.get().clone(),
            NetworkMsg::ScrollDown => network_config.scroll_down.get().clone(),
        };

        if !cmd.is_empty()
            && let Err(e) = spawn_shell_quiet(&cmd)
        {
            error!(error = %e, cmd = %cmd, "failed to spawn command");
        }
    }

    fn update_cmd(&mut self, msg: NetworkCmd, _sender: ComponentSender<Self>, _root: &Self::Root) {
        let config_service = services::get::<ConfigService>();
        let network_config = &config_service.config().modules.network;

        match msg {
            NetworkCmd::StateChanged | NetworkCmd::IconConfigChanged => {
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
                if let Some(wifi) = &network_service.wifi {
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
                if let Some(wired) = &network_service.wired {
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
                if let Some(wifi) = &network_service.wifi {
                    let ssid = wifi.ssid.get();
                    let ctx = WifiContext {
                        enabled: wifi.enabled.get(),
                        connectivity: wifi.connectivity.get(),
                        strength: wifi.strength.get(),
                        ssid: ssid.as_deref(),
                    };
                    (wifi_icon(config, &ctx), wifi_label(&ctx))
                } else if let Some(wired) = &network_service.wired {
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

    fn spawn_watchers(sender: &ComponentSender<Self>, config: &NetworkConfig) {
        let network_service = services::get::<NetworkService>();

        let primary = network_service.primary.clone();
        watch!(sender, [primary.watch()], |out| {
            let _ = out.send(NetworkCmd::StateChanged);
        });

        if let Some(wifi) = &network_service.wifi {
            let enabled = wifi.enabled.clone();
            let connectivity = wifi.connectivity.clone();
            let ssid = wifi.ssid.clone();
            let strength = wifi.strength.clone();
            watch!(
                sender,
                [
                    enabled.watch(),
                    connectivity.watch(),
                    ssid.watch(),
                    strength.watch()
                ],
                |out| {
                    let _ = out.send(NetworkCmd::StateChanged);
                }
            );
        }

        if let Some(wired) = &network_service.wired {
            let connectivity = wired.connectivity.clone();
            watch!(sender, [connectivity.watch()], |out| {
                let _ = out.send(NetworkCmd::StateChanged);
            });
        }

        let wifi_disabled_icon = config.wifi_disabled_icon.clone();
        let wifi_acquiring_icon = config.wifi_acquiring_icon.clone();
        let wifi_offline_icon = config.wifi_offline_icon.clone();
        let wifi_connected_icon = config.wifi_connected_icon.clone();
        let wifi_signal_icons = config.wifi_signal_icons.clone();
        let wired_connected_icon = config.wired_connected_icon.clone();
        let wired_acquiring_icon = config.wired_acquiring_icon.clone();
        let wired_disconnected_icon = config.wired_disconnected_icon.clone();
        watch!(
            sender,
            [
                wifi_disabled_icon.watch(),
                wifi_acquiring_icon.watch(),
                wifi_offline_icon.watch(),
                wifi_connected_icon.watch(),
                wifi_signal_icons.watch(),
                wired_connected_icon.watch(),
                wired_acquiring_icon.watch(),
                wired_disconnected_icon.watch()
            ],
            |out| {
                let _ = out.send(NetworkCmd::IconConfigChanged);
            }
        );
    }
}
