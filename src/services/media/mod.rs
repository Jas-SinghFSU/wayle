/// Core media domain models
pub mod core;
mod error;
mod monitoring;
mod proxy;
mod service;
/// Type definitions for media service configuration, states, and identifiers
pub mod types;

pub use error::Error;
pub use service::MediaService;
