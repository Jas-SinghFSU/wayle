#[derive(Debug)]
pub(crate) struct DeviceInfo {
    pub description: String,
    pub subtitle: Option<String>,
    pub icon: &'static str,
    pub is_active: bool,
}

pub(crate) struct DevicePickerInit {
    pub title: String,
}

#[derive(Debug)]
pub(crate) enum DevicePickerInput {
    SetDevices(Vec<DeviceInfo>),
    DeviceSelected(usize),
    BackClicked,
}

#[derive(Debug)]
pub(crate) enum DevicePickerOutput {
    DeviceSelected(usize),
    NavigateBack,
}
