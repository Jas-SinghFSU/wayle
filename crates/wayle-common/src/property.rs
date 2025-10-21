use std::fmt::Debug;

use futures::stream::{Stream, StreamExt};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tokio::sync::{mpsc, watch};
use tokio_stream::wrappers::WatchStream;
use tracing::warn;

#[cfg(feature = "schema")]
use std::borrow::Cow;

#[cfg(feature = "schema")]
use schemars::{JsonSchema, Schema, SchemaGenerator};

/// Trait for updating struct fields from TOML values.
///
/// Enables dynamic updates from configuration files without losing
/// existing Property watchers. Implement this trait to enable config reloading.
pub trait UpdateFromToml {
    /// Update this value from a TOML value.
    ///
    /// If deserialization fails, implementations should log and skip the update
    /// rather than returning an error, allowing partial updates to succeed.
    fn update_from_toml(&self, value: &toml::Value);
}

/// Trait for subscribing to changes in config structures.
///
/// Enables automatic persistence by watching all fields for changes.
pub trait SubscribeChanges {
    /// Subscribe to changes by sending notifications to the provided channel.
    ///
    /// Spawns background tasks that watch for changes and send () to the channel.
    fn subscribe_changes(&self, tx: mpsc::UnboundedSender<()>);
}

/// Stream of property value changes.
pub type PropertyStream<T> = Box<dyn Stream<Item = T> + Send + Unpin>;

/// A reactive property that can be watched for changes.
///
/// When the value changes, all watchers are notified automatically.
/// Each watcher gets the current value immediately when subscribing.
#[derive(Clone)]
pub struct Property<T: Clone + Send + Sync + 'static> {
    tx: watch::Sender<T>,
    rx: watch::Receiver<T>,
}

impl<T: Clone + Send + Sync + 'static> Property<T> {
    /// Create a new property with an initial value.
    pub fn new(initial: T) -> Self {
        let (tx, rx) = watch::channel(initial);
        Self { tx, rx }
    }

    /// Set the property value.
    ///
    /// **Note**: This method is intended for service implementations only.
    /// External consumers should not call this method directly.
    #[doc(alias = "internal_set")]
    pub fn set(&self, new_value: T)
    where
        T: PartialEq,
    {
        let _ = self.tx.send_if_modified(|current| {
            if *current != new_value {
                *current = new_value;
                return true;
            }

            false
        });
    }

    /// Get the current value.
    ///
    /// Synchronous operation that clones the current value.
    pub fn get(&self) -> T {
        self.rx.borrow().clone()
    }

    /// Watch for changes to this property.
    ///
    /// The stream immediately yields the current value, then yields
    /// whenever the value changes.
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

impl<T> UpdateFromToml for Property<T>
where
    T: Clone + Send + Sync + PartialEq + for<'de> Deserialize<'de> + 'static,
{
    fn update_from_toml(&self, value: &toml::Value) {
        match T::deserialize(value.clone()) {
            Ok(new_value) => {
                self.set(new_value);
            }
            Err(e) => {
                warn!(
                    error = %e,
                    value = ?value,
                    "Failed to deserialize TOML value for Property, skipping update"
                );
            }
        }
    }
}

impl<T: Clone + Send + Sync + 'static> SubscribeChanges for Property<T> {
    fn subscribe_changes(&self, tx: mpsc::UnboundedSender<()>) {
        let mut watch_stream = self.watch();

        tokio::spawn(async move {
            watch_stream.next().await;

            while watch_stream.next().await.is_some() {
                let _ = tx.send(());
            }
        });
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
