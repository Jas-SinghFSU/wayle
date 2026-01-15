//! Common traits for Wayle services.

use std::sync::Arc;

/// Background state monitoring for a service.
pub trait ServiceMonitoring {
    /// Error type for monitoring operations.
    type Error;

    /// Starts monitoring for state changes.
    ///
    /// Sets up necessary watchers or listeners to detect and propagate
    /// state changes to subscribers.
    ///
    /// # Errors
    /// Returns error if monitoring setup fails.
    #[allow(async_fn_in_trait)]
    async fn start_monitoring(&self) -> Result<(), Self::Error>;
}

/// Background state monitoring for a model.
pub trait ModelMonitoring {
    /// Error type for monitoring operations.
    type Error;

    /// Starts monitoring for state changes with shared ownership.
    ///
    /// Arc-wrapped variant of service monitoring, enabling the model
    /// to be shared across multiple owners while receiving updates.
    ///
    /// # Errors
    /// Returns error if monitoring setup fails.
    #[allow(async_fn_in_trait)]
    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error>;
}

/// Static models - fetch once, no monitoring.
pub trait Static {
    /// Error type for static operations.
    type Error;
    /// Context type for static fetching.
    type Context<'a>;

    /// Retrieves a static instance from the provided context.
    ///
    /// Creates or retrieves an instance using the given context,
    /// without setting up any monitoring.
    ///
    /// # Errors
    /// Returns error if instance creation or retrieval fails.
    #[allow(async_fn_in_trait)]
    async fn get(context: Self::Context<'_>) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
/// Reactive models - can fetch statically OR with live monitoring.
pub trait Reactive {
    /// Error type for reactive operations.
    type Error;
    /// Context type for static fetching.
    type Context<'a>;
    /// Context type for live monitoring.
    type LiveContext<'a>;

    /// Fetches a static snapshot without monitoring.
    #[allow(async_fn_in_trait)]
    async fn get(context: Self::Context<'_>) -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// Fetches with live monitoring and reactive updates.
    #[allow(async_fn_in_trait)]
    async fn get_live(context: Self::LiveContext<'_>) -> Result<Arc<Self>, Self::Error>
    where
        Self: Sized;
}
