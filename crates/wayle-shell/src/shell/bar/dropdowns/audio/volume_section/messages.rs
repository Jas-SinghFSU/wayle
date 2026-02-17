#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VolumeSectionKind {
    Output,
    Input,
}

pub(crate) struct VolumeSectionInit {
    pub kind: VolumeSectionKind,
    pub title: String,
    pub device_name: String,
    pub device_icon: &'static str,
    pub volume: f64,
    pub muted: bool,
    pub has_device: bool,
}

#[derive(Debug)]
pub(crate) enum VolumeSectionInput {
    SetDevice {
        name: String,
        icon: &'static str,
        volume: f64,
        muted: bool,
    },
    SetVolume(f64),
    SetMuted(bool),
    SetHasDevice(bool),
    VolumeCommitted(f64),
    MuteClicked,
    ShowDevicesClicked,
}

#[derive(Debug)]
pub(crate) enum VolumeSectionOutput {
    VolumeChanged(f64),
    ToggleMute,
    ShowDevices,
}
