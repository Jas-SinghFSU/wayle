//! CAVA audio visualization service for Wayle.

mod error;
mod ffi;
mod monitoring;
mod service;

/// Public types for configuring CAVA visualization.
pub mod types;

pub use error::{Error, Result};
pub use service::{CavaService, CavaServiceBuilder};
pub use types::InputMethod;
