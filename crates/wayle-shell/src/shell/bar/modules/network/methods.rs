use wayle_config::schemas::modules::NetworkConfig;
use wayle_network::{NetworkService, types::connectivity::ConnectionType};

use super::{
    NetworkModule,
    helpers::{WifiContext, WiredContext, wifi_icon, wifi_label, wired_icon, wired_label},
};
use crate::i18n::t;

impl NetworkModule {
    pub(super) fn compute_display(
        config: &NetworkConfig,
        network: &NetworkService,
    ) -> (String, String) {
        let primary = network.primary.get();

        match primary {
            ConnectionType::Wifi => {
                if let Some(wifi) = network.wifi.get() {
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
                        t!("bar-network-no-wifi"),
                    )
                }
            }
            ConnectionType::Wired => {
                if let Some(wired) = network.wired.get() {
                    let ctx = WiredContext {
                        connectivity: wired.connectivity.get(),
                    };
                    (wired_icon(config, &ctx), wired_label(&ctx))
                } else {
                    (
                        config.wired_disconnected_icon.get().clone(),
                        t!("bar-network-no-ethernet"),
                    )
                }
            }
            ConnectionType::None => (
                config.wifi_offline_icon.get().clone(),
                t!("bar-network-offline"),
            ),

            _ => {
                if let Some(wifi) = network.wifi.get() {
                    let ssid = wifi.ssid.get();
                    let ctx = WifiContext {
                        enabled: wifi.enabled.get(),
                        connectivity: wifi.connectivity.get(),
                        strength: wifi.strength.get(),
                        ssid: ssid.as_deref(),
                    };
                    (wifi_icon(config, &ctx), wifi_label(&ctx))
                } else if let Some(wired) = network.wired.get() {
                    let ctx = WiredContext {
                        connectivity: wired.connectivity.get(),
                    };
                    (wired_icon(config, &ctx), wired_label(&ctx))
                } else {
                    (
                        config.wifi_offline_icon.get().clone(),
                        t!("bar-network-offline"),
                    )
                }
            }
        }
    }
}
