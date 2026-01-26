//! Bar container component for modules with custom children.
//!
//! Unlike [`BarButton`](super::bar_buttons::BarButton) which provides a clickable
//! button with icon/label, `BarContainer` is a passthrough container for modules
//! that need to host multiple interactive children (e.g., systray icons).

mod component;
mod styling;
mod types;

pub use component::{BarContainer, BarContainerCmd, BarContainerInput};
pub use types::{BarContainerBehavior, BarContainerClass, BarContainerColors, BarContainerInit};
