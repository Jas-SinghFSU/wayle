/// Core power profiles domain models
pub mod core;
mod error;
mod proxy;
mod service;
/// Power profiles type definitions  
pub mod types;

pub use core::PowerProfiles;

pub use error::Error;
pub use service::PowerProfilesService;
