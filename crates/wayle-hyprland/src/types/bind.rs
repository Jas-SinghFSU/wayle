use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BindData {
    pub locked: bool,
    pub mouse: bool,
    pub release: bool,
    pub repeat: bool,
    pub long_press: bool,
    pub non_consuming: bool,
    pub has_description: bool,
    pub modmask: u32,
    pub submap: String,
    pub key: String,
    pub keycode: i32,
    pub catch_all: bool,
    pub description: String,
    pub dispatcher: String,
    pub arg: String,
}
