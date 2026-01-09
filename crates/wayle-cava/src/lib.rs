//! CAVA audio visualization service for Wayle.

mod builder;
mod error;
mod ffi;
mod monitoring;
mod service;

/// Public types for configuring CAVA visualization.
pub mod types;

pub use builder::CavaServiceBuilder;
pub use error::{Error, Result};
pub use service::CavaService;
pub use types::InputMethod;
