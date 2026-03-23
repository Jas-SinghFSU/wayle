//! VPN provider adapters that bridge generic tunnel UI with specific backends.

use std::sync::Arc;

use wayle_network::{NetworkService, types::flags::NMConnectionSettingsFlags};

use super::messages::{TunnelState, VpnProviderConfig};
use crate::i18n::t;

/// Creates a WireGuard provider configuration.
pub(in crate::shell::bar::dropdowns::network) fn wireguard_config() -> VpnProviderConfig {
    VpnProviderConfig {
        section_label: t!("dropdown-network-wireguard-tunnels"),
        fallback_name: "WireGuard",
        icon: "md-vpn_lock_2-symbolic",
        import_filter: "*.conf",
        import_filter_label: "WireGuard Config (*.conf)",
    }
}

/// Reads the current WireGuard tunnel list as generic `TunnelState` snapshots.
pub(super) fn wireguard_tunnels(network: &Arc<NetworkService>) -> Vec<TunnelState> {
    let config = wireguard_config();
    let Some(wg) = network.wireguard.get() else {
        return vec![];
    };

    let mut tunnels: Vec<TunnelState> = wg
        .tunnels
        .get()
        .iter()
        .map(|t| {
            let flags = t.profile.flags.get();
            tunnel_state(
                t.profile.uuid.get(),
                t.profile.id.get(),
                t.active.get(),
                flags.contains(NMConnectionSettingsFlags::EXTERNAL),
                t.ip4_address.get(),
                t.interface_name.get(),
                t.profile.object_path.clone(),
                config.fallback_name,
            )
        })
        .collect();

    tunnels.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    tunnels
}

/// Checks whether the WireGuard backend service is available.
pub(super) fn wireguard_available(network: &Arc<NetworkService>) -> bool {
    network.wireguard.get().is_some()
}

/// Builds a `TunnelState` with a fallback name chain:
/// connection ID → interface name → provider fallback.
fn tunnel_state(
    uuid: String,
    id: String,
    active: bool,
    externally_managed: bool,
    ip4_address: Option<String>,
    interface_name: Option<String>,
    connection_path: zbus::zvariant::OwnedObjectPath,
    fallback_name: &str,
) -> TunnelState {
    let name = if id.is_empty() {
        interface_name
            .clone()
            .unwrap_or_else(|| String::from(fallback_name))
    } else {
        id
    };

    TunnelState {
        uuid,
        name,
        active,
        externally_managed,
        ip4_address,
        interface_name,
        connection_path,
    }
}
