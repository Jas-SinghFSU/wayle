//! Reusable GTK4 widget components for Wayle.
//!
//! Provides primitive UI building blocks styled according to Wayle's design system.
//!
//! # Quick Start
//!
//! Import everything via the prelude:
//!
//! ```ignore
//! use wayle_widgets::prelude::*;
//! ```
//!
//! Or import specific modules:
//!
//! ```ignore
//! use wayle_widgets::primitives::card::{Card, CardClass};
//! ```

pub mod primitives;

/// Convenient re-exports of all widget templates and class constants.
pub mod prelude {
    pub use crate::primitives::alert::*;
    pub use crate::primitives::badge::*;
    pub use crate::primitives::buttons::*;
    pub use crate::primitives::card::*;
    pub use crate::primitives::checkbox::*;
    pub use crate::primitives::confirm_modal::*;
    pub use crate::primitives::dropdown::*;
    pub use crate::primitives::empty_state::*;
    pub use crate::primitives::popover::*;
    pub use crate::primitives::progress_bar::*;
    pub use crate::primitives::progress_ring::*;
    pub use crate::primitives::radio_group::*;
    pub use crate::primitives::separator::*;
    pub use crate::primitives::slider::*;
    pub use crate::primitives::spinner::*;
    pub use crate::primitives::status_dot::*;
    pub use crate::primitives::switch::*;
    pub use crate::primitives::text_input::*;
}
