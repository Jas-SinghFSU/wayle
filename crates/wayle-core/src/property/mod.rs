//! Single-producer, multi-consumer reactive values built on
//! [`tokio::sync::watch`]. See [`Property`] for the main type.

mod serde;
mod stream;

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use futures::stream::Stream;
use tokio::sync::{Notify, watch};
use tokio_stream::wrappers::WatchStream;

use self::stream::SubscribedStream;

/// Stream of property value changes.
pub type PropertyStream<T> = Box<dyn Stream<Item = T> + Send + Unpin>;

/// A value you can `.get()` or `.watch()` for changes.
///
/// ```ignore
/// let volume = device.volume.get();
///
/// let mut changes = device.volume.watch();
/// while let Some(vol) = changes.next().await {
///     println!("{vol:?}");
/// }
/// ```
#[derive(Clone)]
pub struct Property<T: Clone + Send + Sync + 'static> {
    tx: watch::Sender<T>,
    rx: watch::Receiver<T>,

    subscriber_count: Arc<AtomicUsize>,
    subscriber_notify: Arc<Notify>,
}

impl<T: Clone + Send + Sync + 'static> Property<T> {
    /// Creates a property with an initial value.
    ///
    /// ```
    /// use wayle_core::Property;
    ///
    /// let temperature = Property::new(22.5_f64);
    /// assert_eq!(temperature.get(), 22.5);
    /// ```
    #[doc(hidden)]
    pub fn new(initial: T) -> Self {
        let (tx, rx) = watch::channel(initial);

        Self {
            tx,
            rx,
            subscriber_count: Arc::new(AtomicUsize::new(0)),
            subscriber_notify: Arc::new(Notify::new()),
        }
    }

    /// Updates the value. Watchers are only notified if the value actually changed.
    ///
    /// ```
    /// use wayle_core::Property;
    ///
    /// let volume = Property::new(50_u32);
    ///
    /// volume.set(75);
    /// assert_eq!(volume.get(), 75);
    ///
    /// // Setting the same value is a no-op (no watcher notification).
    /// volume.set(75);
    /// ```
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

    /// Unconditional [`set`](Self::set). Always notifies watchers, and
    /// doesn't require `PartialEq`.
    ///
    /// ```
    /// use wayle_core::Property;
    ///
    /// let data = Property::new(vec![1, 2, 3]);
    /// data.replace(vec![1, 2, 3]);
    /// // Watchers fire even though the Vec is equal.
    /// // Use `set` instead if you want to skip duplicates.
    /// ```
    pub fn replace(&self, new_value: T) {
        self.tx.send_modify(|current| *current = new_value);
    }

    /// Snapshot of the current value (cloned).
    ///
    /// ```
    /// use wayle_core::Property;
    ///
    /// let name = Property::new(String::from("default"));
    /// assert_eq!(name.get(), "default");
    /// ```
    pub fn get(&self) -> T {
        self.rx.borrow().clone()
    }

    /// Yields the current value immediately, then each subsequent change.
    ///
    /// Each call returns an independent stream. Multiple consumers
    /// can watch the same property concurrently.
    ///
    /// ```no_run
    /// use futures::stream::StreamExt;
    /// use wayle_core::Property;
    ///
    /// # async fn example() {
    /// let score = Property::new(0_u32);
    ///
    /// let mut stream = score.watch();
    /// while let Some(points) = stream.next().await {
    ///     println!("score: {points}");
    /// }
    /// # }
    /// ```
    pub fn watch(&self) -> impl Stream<Item = T> + Send + 'static {
        SubscribedStream::new(
            WatchStream::new(self.rx.clone()),
            Arc::clone(&self.subscriber_count),
            Arc::clone(&self.subscriber_notify),
        )
    }

    /// Whether any [`.watch()`](Self::watch) streams are alive.
    pub fn has_subscribers(&self) -> bool {
        self.subscriber_count.load(Ordering::Acquire) > 0
    }

    /// Suspends until at least one consumer calls [`.watch()`](Self::watch).
    ///
    /// Pair with [`has_subscribers`](Self::has_subscribers) in a loop to
    /// pause expensive work whenever nobody is listening, and resume
    /// when someone subscribes again.
    ///
    /// ```no_run
    /// use wayle_core::Property;
    ///
    /// # async fn example() {
    /// let cpu_usage = Property::new(0.0_f64);
    ///
    /// loop {
    ///     if !cpu_usage.has_subscribers() {
    ///         cpu_usage.wait_for_subscribers().await;
    ///     }
    ///
    ///     let usage = 42.0; // poll_cpu();
    ///     cpu_usage.set(usage);
    /// }
    /// # }
    /// ```
    pub async fn wait_for_subscribers(&self) {
        while !self.has_subscribers() {
            self.subscriber_notify.notified().await;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::task::Poll;

    use futures::{poll, stream::StreamExt};

    use super::*;

    #[test]
    fn set_updates_value() {
        let property = Property::new(42);
        property.set(100);

        assert_eq!(property.get(), 100);
    }

    #[tokio::test]
    async fn set_skips_notification_when_unchanged() {
        let property = Property::new(42);
        let mut stream = property.watch();

        assert_eq!(stream.next().await, Some(42));

        property.set(42);
        assert_eq!(poll!(stream.next()), Poll::Pending);
    }

    #[tokio::test]
    async fn notifies_watchers_on_change() {
        let property = Property::new(1);
        let mut stream = property.watch();

        assert_eq!(stream.next().await, Some(1));

        property.set(2);
        assert_eq!(stream.next().await, Some(2));
    }

    #[test]
    fn no_subscribers_initially() {
        let property = Property::new(0);

        assert!(!property.has_subscribers());
    }

    #[test]
    fn tracks_subscriber_lifetime() {
        let property = Property::new(0);

        let stream = property.watch();
        assert!(property.has_subscribers());

        drop(stream);
        assert!(!property.has_subscribers());
    }

    #[test]
    fn tracks_multiple_subscribers() {
        let property = Property::new(0);

        let stream_a = property.watch();
        let stream_b = property.watch();
        let stream_c = property.watch();
        assert!(property.has_subscribers());

        drop(stream_a);
        assert!(property.has_subscribers());

        drop(stream_b);
        assert!(property.has_subscribers());

        drop(stream_c);
        assert!(!property.has_subscribers());
    }

    #[test]
    fn clones_share_subscriber_count() {
        let property = Property::new(0);
        let cloned = property.clone();

        let _stream = cloned.watch();

        assert!(property.has_subscribers());
    }

    #[tokio::test]
    async fn wait_for_subscribers_resolves_on_first_watcher() {
        let property = Property::new(0);
        let waiting = property.clone();

        let (ready_tx, ready_rx) = tokio::sync::oneshot::channel::<()>();

        let waiter = tokio::spawn(async move {
            let _ = ready_tx.send(());
            waiting.wait_for_subscribers().await;
        });

        ready_rx.await.unwrap();
        let _stream = property.watch();

        waiter.await.unwrap();
    }

    #[tokio::test]
    async fn wait_for_subscribers_returns_immediately_if_already_watched() {
        let property = Property::new(0);
        let _stream = property.watch();

        property.wait_for_subscribers().await;
    }
}
