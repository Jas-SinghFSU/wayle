use std::sync::Arc;

use relm4::ComponentSender;
use tokio_util::sync::CancellationToken;
use wayle_bluetooth::BluetoothService;
use wayle_common::{Property, watch, watch_cancellable};
use wayle_network::{NetworkService, wifi::Wifi};
use wayle_notification::NotificationService;
use wayle_power_profiles::{PowerProfilesService, types::profile::PowerProfile};

use super::{QuickActionsSection, messages::QuickActionsCmd};
use crate::services::IdleInhibitService;

pub(super) fn spawn(
    sender: &ComponentSender<QuickActionsSection>,
    network: &Option<Arc<NetworkService>>,
    bluetooth: &Option<Arc<BluetoothService>>,
    notification: &Option<Arc<NotificationService>>,
    power_profiles: &Property<Option<Arc<PowerProfilesService>>>,
    idle_inhibit: &Arc<IdleInhibitService>,
) {
    if let Some(network) = network {
        let wifi_prop = network.wifi.clone();

        watch!(sender, [wifi_prop.watch()], |out| {
            let has_wifi = wifi_prop.get().is_some();
            let _ = out.send(QuickActionsCmd::WifiAvailabilityChanged(has_wifi));
        });
    }

    if let Some(bluetooth) = bluetooth {
        let enabled = bluetooth.enabled.clone();

        watch!(sender, [enabled.watch()], |out| {
            let _ = out.send(QuickActionsCmd::BluetoothChanged(enabled.get()));
        });

        let available = bluetooth.available.clone();

        watch!(sender, [available.watch()], |out| {
            let _ = out.send(QuickActionsCmd::BluetoothAvailabilityChanged(
                available.get(),
            ));
        });
    }

    if let Some(notification) = notification {
        let dnd = notification.dnd.clone();

        watch!(sender, [dnd.watch()], |out| {
            let _ = out.send(QuickActionsCmd::DndChanged(dnd.get()));
        });
    }

    let active = idle_inhibit.state().active.clone();

    watch!(sender, [active.watch()], |out| {
        let _ = out.send(QuickActionsCmd::IdleInhibitChanged(active.get()));
    });

    spawn_power_profile_availability(sender, power_profiles);
}

pub(super) fn spawn_power_profile_availability(
    sender: &ComponentSender<QuickActionsSection>,
    property: &Property<Option<Arc<PowerProfilesService>>>,
) {
    let property = property.clone();

    watch!(sender, [property.watch()], |out| {
        match property.get() {
            Some(service) => {
                let _ = out.send(QuickActionsCmd::PowerProfilesAvailable(service));
            }
            None => {
                let _ = out.send(QuickActionsCmd::PowerProfilesUnavailable);
            }
        }
    });
}

pub(super) fn spawn_power_profile_watcher(
    sender: &ComponentSender<QuickActionsSection>,
    service: &Arc<PowerProfilesService>,
    token: CancellationToken,
) {
    let profile = service.power_profiles.active_profile.clone();

    watch_cancellable!(sender, token, [profile.watch()], |out| {
        let is_saver = profile.get() == PowerProfile::PowerSaver;
        let _ = out.send(QuickActionsCmd::PowerSaverChanged(is_saver));
    });
}

pub(super) fn spawn_wifi_enabled_watcher(
    sender: &ComponentSender<QuickActionsSection>,
    wifi: &Arc<Wifi>,
    token: CancellationToken,
) {
    let enabled = wifi.enabled.clone();

    watch_cancellable!(sender, token, [enabled.watch()], |out| {
        let _ = out.send(QuickActionsCmd::WifiChanged(enabled.get()));
    });
}
