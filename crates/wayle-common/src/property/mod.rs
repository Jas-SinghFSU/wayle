//! Reactive property system with layered configuration support.
//!
//! Reactive properties that can be watched for changes, with special support
//! for three-layer configuration (default, config, runtime).
//!
//! # Data Flow
//!
//! ```text
//! config.toml --> ApplyConfigLayer --> ConfigProperty.config
//!                                              |
//!                                              v
//! runtime.toml --> ApplyRuntimeLayer --> ConfigProperty.runtime --> effective value
//!                                              |
//!                                              v
//!                                      SubscribeChanges --> watchers
//!
//! On save:
//!
//! ConfigProperty.runtime --> ExtractRuntimeValues --> runtime.toml
//! ```
//!
//! # Layer Precedence
//!
//! The effective value follows: `runtime > config > default`
//!
//! - **Default**: Compiled-in value, always present
//! - **Config**: User's base configuration from `config.toml`
//! - **Runtime**: GUI overrides persisted to `runtime.toml`
//!
//! # Value Source
//!
//! [`ValueSource`] indicates where the effective value originates:
//!
//! - `Default` - Using compiled default (no config, no runtime)
//! - `Config` - Using config.toml value (has config, no runtime)
//! - `Custom` - Using runtime value without config.toml base (no config, has runtime)
//! - `Override` - Runtime override of config.toml value (has config, has runtime)
//!
//! # Traits
//!
//! - [`ApplyConfigLayer`] - Apply TOML values to the config layer
//! - [`ApplyRuntimeLayer`] - Apply TOML values to the runtime layer
//! - [`ExtractRuntimeValues`] - Extract runtime overrides for persistence
//! - [`SubscribeChanges`] - Subscribe to value change notifications
//!
//! All traits have derive macros in `wayle_derive` for automatic implementation
//! on config structs.

mod config;
mod traits;

#[cfg(feature = "schema")]
use std::borrow::Cow;
use std::fmt::Debug;

pub use config::{ConfigProperty, ValueSource};
use futures::stream::{Stream, StreamExt};
#[cfg(feature = "schema")]
use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tokio::sync::watch;
use tokio_stream::wrappers::WatchStream;
pub use traits::{
    ApplyConfigLayer, ApplyRuntimeLayer, ClearRuntimeByPath, CommitConfigReload,
    ExtractRuntimeValues, ResetConfigLayer, ResetRuntimeLayer, SubscribeChanges,
};

/// Stream of property value changes.
pub type PropertyStream<T> = Box<dyn Stream<Item = T> + Send + Unpin>;

/// Reactive property exposing service state.
///
/// # Reading State
///
/// - `.get()` - Returns the current value (snapshot)
/// - `.watch()` - Returns a stream that yields on every change
///
/// ```ignore
/// // Snapshot
/// let volume = device.volume.get();
///
/// // React to changes
/// let mut stream = device.volume.watch();
/// while let Some(vol) = stream.next().await {
///     println!("Volume: {vol:?}");
/// }
/// ```
#[derive(Clone)]
pub struct Property<T: Clone + Send + Sync + 'static> {
    tx: watch::Sender<T>,
    rx: watch::Receiver<T>,
}

impl<T: Clone + Send + Sync + 'static> Property<T> {
    /// Creates a property with an initial value.
    #[doc(hidden)]
    pub fn new(initial: T) -> Self {
        let (tx, rx) = watch::channel(initial);
        Self { tx, rx }
    }

    /// Sets the property value, notifying watchers if changed.
    #[doc(hidden)]
    pub fn set(&self, new_value: T)
    where
        T: PartialEq,
    {
        self.tx.send_if_modified(|current| {
            if *current != new_value {
                *current = new_value;
                return true;
            }

            false
        });
    }

    /// Returns the current value.
    pub fn get(&self) -> T {
        self.rx.borrow().clone()
    }

    /// Watches for value changes.
    ///
    /// Yields the current value immediately, then on every change.
    pub fn watch(&self) -> impl Stream<Item = T> + Send + 'static {
        WatchStream::new(self.rx.clone())
    }
}

impl<T: Clone + Send + Sync + Debug + 'static> Debug for Property<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Property")
            .field("value", &self.get())
            .finish()
    }
}

impl<T: Clone + Send + Sync + Serialize + 'static> Serialize for Property<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.get().serialize(serializer)
    }
}

impl<'de, T: Clone + Send + Sync + Deserialize<'de> + 'static> Deserialize<'de> for Property<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Ok(Property::new(value))
    }
}

#[cfg(feature = "schema")]
impl<T: Clone + Send + Sync + JsonSchema + 'static> JsonSchema for Property<T> {
    fn schema_name() -> Cow<'static, str> {
        T::schema_name()
    }

    fn json_schema(gen_param: &mut SchemaGenerator) -> Schema {
        T::json_schema(gen_param)
    }
}

/// Create a property that derives its value from other properties.
///
/// The computed property automatically updates when any dependency changes.
pub struct ComputedProperty<T: Clone + Send + Sync + 'static> {
    property: Property<T>,
    _task: tokio::task::JoinHandle<()>,
}

impl<T: Clone + Send + Sync + 'static> ComputedProperty<T> {
    /// Create a new computed property.
    ///
    /// The computation function is called whenever any input stream yields a value.
    pub fn new<S, F>(initial: T, mut inputs: S, mut compute: F) -> Self
    where
        S: Stream + Send + Unpin + 'static,
        F: FnMut() -> T + Send + 'static,
        T: PartialEq + Sync,
    {
        let property = Property::new(initial);
        let prop_clone = property.clone();

        let task = tokio::spawn(async move {
            while inputs.next().await.is_some() {
                let new_value = compute();
                prop_clone.set(new_value);
            }
        });

        Self {
            property,
            _task: task,
        }
    }

    /// Get the current computed value.
    pub fn get(&self) -> T {
        self.property.get()
    }

    /// Watch for changes to the computed value.
    pub fn watch(&self) -> impl Stream<Item = T> + Send + 'static {
        self.property.watch()
    }
}

impl<T: Clone + Send + Sync + 'static> Drop for ComputedProperty<T> {
    fn drop(&mut self) {
        self._task.abort();
    }
}

#[cfg(test)]
mod tests {
    use futures::stream;
    use tokio::sync::mpsc;

    use super::*;

    #[test]
    fn set_updates_value_when_different() {
        let property = Property::new(42);

        property.set(100);

        assert_eq!(property.get(), 100);
    }

    #[test]
    fn set_does_not_notify_when_value_unchanged() {
        let property = Property::new(42);
        let mut watch_stream = property.watch();

        property.set(42);

        let current_value = tokio::runtime::Runtime::new().unwrap().block_on(async {
            tokio::time::timeout(tokio::time::Duration::from_millis(10), watch_stream.next()).await
        });

        assert!(current_value.is_ok());
        assert_eq!(current_value.unwrap().unwrap(), 42);

        property.set(42);

        let next_value = tokio::runtime::Runtime::new().unwrap().block_on(async {
            tokio::time::timeout(tokio::time::Duration::from_millis(10), watch_stream.next()).await
        });

        assert!(next_value.is_err());
    }

    #[tokio::test]
    async fn set_notifies_watchers_when_value_changes() {
        let property = Property::new(1);
        let mut watch_stream = property.watch();

        let initial = watch_stream.next().await;
        assert_eq!(initial, Some(1));

        property.set(2);

        let updated = watch_stream.next().await;
        assert_eq!(updated, Some(2));
    }

    #[tokio::test]
    async fn computed_property_initializes_with_initial_value() {
        let (tx, mut rx) = mpsc::channel::<()>(1);
        let input_stream = stream::poll_fn(move |cx| rx.poll_recv(cx));

        let computed = ComputedProperty::new(10, input_stream, || 20);

        assert_eq!(computed.get(), 10);

        drop(tx);
    }

    #[tokio::test]
    async fn computed_property_recomputes_when_input_stream_emits() {
        let (tx, mut rx) = mpsc::channel(10);
        let input_stream = stream::poll_fn(move |cx| rx.poll_recv(cx));

        let counter = std::sync::Arc::new(std::sync::atomic::AtomicI32::new(0));
        let counter_clone = counter.clone();

        let computed = ComputedProperty::new(0, input_stream, move || {
            counter_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });

        assert_eq!(computed.get(), 0);

        tx.send(()).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        assert_eq!(computed.get(), 0);

        tx.send(()).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        assert_eq!(computed.get(), 1);

        drop(tx);
    }

    #[tokio::test]
    async fn computed_property_stops_computing_when_stream_ends() {
        let (tx, mut rx) = mpsc::channel(10);
        let input_stream = stream::poll_fn(move |cx| rx.poll_recv(cx));

        let counter = std::sync::Arc::new(std::sync::atomic::AtomicI32::new(0));
        let counter_clone = counter.clone();

        let computed = ComputedProperty::new(0, input_stream, move || {
            counter_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });

        tx.send(()).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        let value_before_drop = computed.get();

        drop(tx);
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        assert_eq!(computed.get(), value_before_drop);
    }

    #[tokio::test]
    async fn computed_property_aborts_task_on_drop() {
        let (tx, mut rx) = mpsc::channel(10);
        let input_stream = stream::poll_fn(move |cx| rx.poll_recv(cx));

        let counter = std::sync::Arc::new(std::sync::atomic::AtomicI32::new(0));
        let counter_clone = counter.clone();

        {
            let computed = ComputedProperty::new(0, input_stream, move || {
                counter_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            });

            tx.send(()).await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

            let value = computed.get();
            assert_eq!(value, 0);
        }

        let count_after_drop = counter.load(std::sync::atomic::Ordering::SeqCst);

        tx.send(()).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        let count_after_send = counter.load(std::sync::atomic::Ordering::SeqCst);
        assert_eq!(count_after_send, count_after_drop);
    }

    #[test]
    fn serializes_to_inner_value() {
        let property = Property::new(42);

        let json = serde_json::to_string(&property).unwrap();

        assert_eq!(json, "42");
    }

    #[test]
    fn deserializes_from_inner_value() {
        let json = "\"hello\"";

        let property: Property<String> = serde_json::from_str(json).unwrap();

        assert_eq!(property.get(), "hello");
    }

    #[test]
    fn roundtrip_serialization() {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Config {
            name: Property<String>,
            count: Property<i32>,
            enabled: Property<bool>,
        }

        let config = Config {
            name: Property::new(String::from("test")),
            count: Property::new(42),
            enabled: Property::new(true),
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name.get(), "test");
        assert_eq!(deserialized.count.get(), 42);
        assert!(deserialized.enabled.get());
    }

    #[test]
    fn toml_serialization() {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct ClockConfig {
            format: Property<String>,
            show_seconds: Property<bool>,
        }

        let config = ClockConfig {
            format: Property::new(String::from("%H:%M")),
            show_seconds: Property::new(false),
        };

        let toml_string = toml::to_string(&config).unwrap();

        assert!(toml_string.contains("format = \"%H:%M\""));
        assert!(toml_string.contains("show_seconds = false"));
    }

    #[test]
    fn toml_deserialization() {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct ClockConfig {
            format: Property<String>,
            show_seconds: Property<bool>,
        }

        let toml_string = r#"
            format = "%H:%M:%S"
            show_seconds = true
        "#;

        let config: ClockConfig = toml::from_str(toml_string).unwrap();

        assert_eq!(config.format.get(), "%H:%M:%S");
        assert!(config.show_seconds.get());
    }
}
