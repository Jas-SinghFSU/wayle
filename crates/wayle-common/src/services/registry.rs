//! Type-map based service registry for dependency injection.

#![allow(clippy::panic)]

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

/// Thread-safe service registry using the type-map pattern.
///
/// Services are registered once at startup and accessed by type throughout
/// the application lifetime.
#[derive(Default, Debug)]
pub struct ServiceRegistry {
    services: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl ServiceRegistry {
    /// Creates an empty service registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a service by its type.
    pub fn register<S: Send + Sync + 'static>(&mut self, service: S) -> &mut Self {
        self.services.insert(TypeId::of::<S>(), Arc::new(service));
        self
    }

    /// Registers a service already wrapped in Arc.
    ///
    /// Useful for services whose constructors return `Arc<Self>`.
    pub fn register_arc<S: Send + Sync + 'static>(&mut self, service: Arc<S>) -> &mut Self {
        self.services.insert(TypeId::of::<S>(), service);
        self
    }

    /// Returns a service by type.
    ///
    /// # Panics
    ///
    /// Panics if the service is not registered.
    pub fn get<S: Send + Sync + 'static>(&self) -> Arc<S> {
        self.services
            .get(&TypeId::of::<S>())
            .and_then(|s| s.clone().downcast::<S>().ok())
            .unwrap_or_else(|| panic!("Service `{}` not registered", std::any::type_name::<S>()))
    }

    /// Attempts to retrieve a service by type.
    ///
    /// Returns `None` if the service is not registered. Useful for optional services.
    pub fn try_get<S: Send + Sync + 'static>(&self) -> Option<Arc<S>> {
        self.services
            .get(&TypeId::of::<S>())
            .and_then(|s| s.clone().downcast::<S>().ok())
    }
}
