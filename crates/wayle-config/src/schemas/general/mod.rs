mod bar;

pub use bar::*;
use wayle_derive::wayle_config;

/// General configuration settings for the Wayle application.
///
/// Contains global settings that don't affect styling.
#[wayle_config]
pub struct GeneralConfig {
    /// Bar configuration.
    pub bar: BarConfig,
}
