use std::fmt::Debug;

use futures::stream::{Stream, StreamExt};
use tokio::sync::watch;
use tokio_stream::wrappers::WatchStream;

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
}
