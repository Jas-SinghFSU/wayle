use std::sync::Arc;

use wayle_bluetooth::BluetoothService;
use wayle_common::Property;
use wayle_network::NetworkService;
use wayle_notification::NotificationService;
use wayle_power_profiles::PowerProfilesService;

use crate::services::IdleInhibitService;

pub(crate) struct QuickActionsInit {
    pub network: Option<Arc<NetworkService>>,
    pub bluetooth: Option<Arc<BluetoothService>>,
    pub notification: Option<Arc<NotificationService>>,
    pub power_profiles: Property<Option<Arc<PowerProfilesService>>>,
    pub idle_inhibit: Arc<IdleInhibitService>,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum QuickActionsInput {
    WifiToggled,
    BluetoothToggled,
    AirplaneToggled,
    DndToggled,
    IdleInhibitToggled,
    PowerSaverToggled,
}

#[derive(Debug)]
pub(crate) enum QuickActionsCmd {
    WifiChanged(bool),
    WifiAvailabilityChanged(bool),
    BluetoothChanged(bool),
    BluetoothAvailabilityChanged(bool),
    DndChanged(bool),
    IdleInhibitChanged(bool),
    PowerSaverChanged(bool),
    PowerProfilesAvailable(Arc<PowerProfilesService>),
    PowerProfilesUnavailable,
}
