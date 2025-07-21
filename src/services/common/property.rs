use std::fmt::Debug;

use futures::stream::{Stream, StreamExt};
use tokio::sync::watch;
use tokio_stream::wrappers::WatchStream;

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

    /// Set a new value and notify all watchers.
    ///
    /// Only updates if the value is different (requires PartialEq).
    /// Only accessible within the crate to prevent external modification.
    pub(crate) fn set(&self, new_value: T)
    where
        T: PartialEq,
    {
        let _ = self.tx.send_if_modified(|current| {
            if *current != new_value {
                *current = new_value;
                true
            } else {
                false
            }
        });
    }

    /// Get the current value.
    ///
    /// This is a synchronous operation that clones the current value.
    pub fn get(&self) -> T {
        self.rx.borrow().clone()
    }

    /// Watch for changes to this property.
    ///
    /// The stream immediately yields the current value, then yields
    /// whenever the value changes.
    pub fn watch(&self) -> impl Stream<Item = T> + Send {
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
    pub fn watch(&self) -> impl Stream<Item = T> + Send {
        self.property.watch()
    }
}

impl<T: Clone + Send + Sync + 'static> Drop for ComputedProperty<T> {
    fn drop(&mut self) {
        self._task.abort();
    }
}
