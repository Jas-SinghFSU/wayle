//! Domain types for the wallpaper service.

mod color_extractor;
mod cycling;
mod fit_mode;
mod monitor_state;

pub use color_extractor::ColorExtractor;
pub use cycling::{CyclingConfig, CyclingMode};
pub use fit_mode::FitMode;
pub use monitor_state::MonitorState;
