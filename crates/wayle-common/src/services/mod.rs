//! Global service registry for application-wide dependency injection.
//!
//! Initialize once at startup, then access services from anywhere in the app.
//!
//! # Example
//!
//! ```ignore
//! // At startup
//! let mut registry = ServiceRegistry::new();
//! registry.register(WallpaperService::new()?);
//! registry.register(MediaService::new()?);
//! services::init(registry);
//!
//! // Anywhere in the app
//! let wallpaper = services::get::<WallpaperService>();
//! ```

#![allow(clippy::expect_used)]

mod registry;

use std::sync::{Arc, OnceLock};

pub use registry::ServiceRegistry;

static SERVICES: OnceLock<ServiceRegistry> = OnceLock::new();

/// Initializes the global service registry.
///
/// Must be called once at application startup before any components access services.
///
/// # Panics
///
/// Panics if called more than once.
pub fn init(registry: ServiceRegistry) {
    SERVICES
        .set(registry)
        .expect("Service registry already initialized");
}

/// Returns a reference to the global service registry.
///
/// # Panics
///
/// Panics if `init()` has not been called.
pub fn registry() -> &'static ServiceRegistry {
    SERVICES
        .get()
        .expect("Service registry not initialized - call services::init() first")
}

/// Retrieves a service by type from the global registry.
///
/// # Panics
///
/// Panics if the registry is not initialized or the service is not registered.
pub fn get<S: Send + Sync + 'static>() -> Arc<S> {
    registry().get::<S>()
}

/// Attempts to retrieve a service by type from the global registry.
///
/// Returns `None` if the service is not registered.
///
/// # Panics
///
/// Panics if the registry is not initialized.
pub fn try_get<S: Send + Sync + 'static>() -> Option<Arc<S>> {
    registry().try_get::<S>()
}
