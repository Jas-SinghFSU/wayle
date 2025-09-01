use std::sync::Arc;

/// Background state monitoring for a service
pub(crate) trait ServiceMonitoring {
    type Error;

    #[allow(async_fn_in_trait)]
    async fn start_monitoring(&self) -> Result<(), Self::Error>;
}

/// Background state monitoring for a model
pub(crate) trait ModelMonitoring {
    type Error;

    #[allow(async_fn_in_trait)]
    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error>;
}

/// Static models - fetch once, no monitoring
pub(crate) trait Static {
    type Error;
    type Context<'a>;

    #[allow(async_fn_in_trait)]
    async fn get(context: Self::Context<'_>) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
/// Reactive models - can fetch statically OR with live monitoring
pub(crate) trait Reactive {
    type Error;
    type Context<'a>;
    type LiveContext<'a>;

    /// Static fetch without monitoring
    #[allow(async_fn_in_trait)]
    async fn get(context: Self::Context<'_>) -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// Live monitoring with reactive updates
    #[allow(async_fn_in_trait)]
    async fn get_live(context: Self::LiveContext<'_>) -> Result<Arc<Self>, Self::Error>
    where
        Self: Sized;
}
