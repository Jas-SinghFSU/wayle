//! Reusable GTK4 widget components for Wayle.
//!
//! Provides primitive UI building blocks styled according to Wayle's design system.
//!
//! # Quick Start
//!
//! Import everything via the prelude:
//!
//! ```rust,no_run
//! use wayle_widgets::prelude::*;
//! ```
//!
//! Or import specific modules:
//!
//! ```rust,no_run
//! use wayle_widgets::primitives::card::{Card, CardClass};
//! use wayle_widgets::components::bar_buttons::{BarButton, BarButtonOutput};
//! ```

pub mod components;
pub mod primitives;
pub mod styling;

/// Convenient re-exports of all widget templates and class constants.
pub mod prelude {
    pub use crate::{
        components::bar_buttons::*,
        primitives::{
            alert::*, badge::*, buttons::*, card::*, checkbox::*, confirm_modal::*, dropdown::*,
            empty_state::*, popover::*, progress_bar::*, progress_ring::*, radio_group::*,
            separator::*, slider::*, spinner::*, status_dot::*, switch::*, text_input::*,
        },
        styling::{InlineStyling, resolve_color},
    };
}
