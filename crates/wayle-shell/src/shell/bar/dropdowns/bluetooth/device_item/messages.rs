use zbus::zvariant::OwnedObjectPath;

use crate::shell::bar::dropdowns::bluetooth::helpers::DeviceSnapshot;

pub(crate) struct DeviceItemInit {
    pub snapshot: DeviceSnapshot,
}

#[derive(Debug)]
pub(crate) enum DeviceItemInput {
    Clicked,
    Hovered(bool),
    ForgetClicked,
}

#[derive(Debug)]
pub(crate) enum DeviceItemOutput {
    Connect(OwnedObjectPath),
    Disconnect(OwnedObjectPath),
    Forget(OwnedObjectPath),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PendingAction {
    Connecting,
    Disconnecting,
    Forgetting,
}
