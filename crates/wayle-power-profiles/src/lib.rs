//! D-Bus client for power-profiles-daemon with reactive property monitoring.
//!
//! # Reactive Properties
//!
//! The [`PowerProfiles`] struct exposes reactive [`Property`](wayle_common::Property) fields
//! that update automatically when the system's power profile state changes:
//!
//! - `.get()` - Returns a snapshot of the current value
//! - `.watch()` - Returns a stream that emits on every change
//!
//! # Service Fields
//!
//! Access power profile state through [`PowerProfilesService::power_profiles`]:
//!
//! | Field | Type | Description |
//! |-------|------|-------------|
//! | `active_profile` | [`PowerProfile`](types::profile::PowerProfile) | Currently active profile |
//! | `performance_degraded` | [`PerformanceDegradationReason`](types::profile::PerformanceDegradationReason) | Why performance mode is degraded, if at all |
//! | `profiles` | `Vec<Profile>` | Available profiles and their drivers |
//! | `actions` | `Vec<String>` | Daemon-supported actions |
//! | `active_profile_holds` | `Vec<ProfileHold>` | Applications holding a specific profile |
//!
//! # Example
//!
//! ```ignore
//! let service = PowerProfilesService::new().await?;
//!
//! // Get current profile
//! let profile = service.power_profiles.active_profile.get();
//!
//! // Watch for changes
//! let mut stream = service.power_profiles.active_profile.watch();
//! while let Some(profile) = stream.next().await {
//!     println!("Profile changed to: {profile}");
//! }
//! ```

mod builder;
mod error;
mod proxy;
mod service;

pub mod core;
pub mod dbus;
pub mod types;

pub use core::PowerProfiles;

pub use builder::PowerProfilesServiceBuilder;
pub use error::Error;
pub use service::PowerProfilesService;
