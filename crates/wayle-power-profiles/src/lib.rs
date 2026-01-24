//! Power profile management via power-profiles-daemon D-Bus.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use wayle_power_profiles::PowerProfilesService;
//!
//! # async fn example() -> Result<(), wayle_power_profiles::Error> {
//! let service = PowerProfilesService::new().await?;
//!
//! // Check current profile
//! let profile = service.power_profiles.active_profile.get();
//! println!("Current profile: {profile}");
//!
//! // List available profiles
//! for p in service.power_profiles.profiles.get() {
//!     println!("  {} (driver: {})", p.profile, p.driver);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Watching for Changes
//!
//! ```rust,no_run
//! use wayle_power_profiles::PowerProfilesService;
//! use futures::StreamExt;
//!
//! # async fn example() -> Result<(), wayle_power_profiles::Error> {
//! # let service = PowerProfilesService::new().await?;
//! // React to profile changes
//! let mut stream = service.power_profiles.active_profile.watch();
//! while let Some(profile) = stream.next().await {
//!     println!("Profile changed to: {profile}");
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Profile Control
//!
//! ```rust,no_run
//! use wayle_power_profiles::PowerProfilesService;
//! use wayle_power_profiles::types::profile::{PowerProfile, PerformanceDegradationReason};
//!
//! # async fn example() -> Result<(), wayle_power_profiles::Error> {
//! # let service = PowerProfilesService::new().await?;
//! // Switch to performance mode
//! service.power_profiles.set_active_profile(PowerProfile::Performance).await?;
//!
//! // Check if performance is degraded (e.g., thermal throttling)
//! let degraded = service.power_profiles.performance_degraded.get();
//! if degraded != PerformanceDegradationReason::None {
//!     println!("Performance degraded: {degraded}");
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Reactive Properties
//!
//! All fields are [`Property<T>`](wayle_common::Property):
//! - `.get()` - Current value snapshot
//! - `.watch()` - Stream yielding on changes
//!
//! # Service Fields
//!
//! | Field | Type | Description |
//! |-------|------|-------------|
//! | `active_profile` | [`PowerProfile`](types::profile::PowerProfile) | Currently active profile |
//! | `performance_degraded` | [`PerformanceDegradationReason`](types::profile::PerformanceDegradationReason) | Why performance is degraded |
//! | `profiles` | `Vec<Profile>` | Available profiles and their drivers |
//! | `actions` | `Vec<String>` | Daemon-supported actions |
//! | `active_profile_holds` | `Vec<ProfileHold>` | Applications holding a profile |
//!
//! # Configuration
//!
//! | Method | Effect |
//! |--------|--------|
//! | `with_daemon()` | Switch profiles from scripts or other processes |
//!
//! ```rust,no_run
//! use wayle_power_profiles::PowerProfilesService;
//!
//! # async fn example() -> Result<(), wayle_power_profiles::Error> {
//! let service = PowerProfilesService::builder()
//!     .with_daemon()
//!     .build()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! # D-Bus Interface
//!
//! When `with_daemon()` is enabled, the service registers on the session bus.
//!
//! - **Service:** `com.wayle.PowerProfiles1`
//! - **Path:** `/com/wayle/PowerProfiles`
//! - **Interface:** `com.wayle.PowerProfiles1`
//!
//! See [`dbus.md`](https://github.com/Jas-SinghFSU/wayle/blob/master/crates/wayle-power-profiles/dbus.md) for the full interface specification.
//!
//! # Control Methods
//!
//! - [`set_active_profile()`](PowerProfiles::set_active_profile) - Switch power profile

mod builder;
mod error;
mod proxy;
mod service;

/// Reactive power profile state.
pub mod core;
/// D-Bus interface for CLI control.
pub mod dbus;
/// Power profile type definitions.
pub mod types;

pub use core::PowerProfiles;

pub use builder::PowerProfilesServiceBuilder;
pub use error::Error;
pub use service::PowerProfilesService;
