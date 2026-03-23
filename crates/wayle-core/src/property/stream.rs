use std::{
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll},
};

use futures::stream::Stream;
use tokio::sync::Notify;
use tokio_stream::wrappers::WatchStream;

/// Wraps a [`WatchStream`] with RAII subscriber tracking.
/// Increments the counter on creation, decrements on drop.
pub(super) struct SubscribedStream<T> {
    inner: WatchStream<T>,
    count: Arc<AtomicUsize>,
    notify: Arc<Notify>,
}

impl<T: Clone + Send + Sync + 'static> SubscribedStream<T> {
    pub(super) fn new(inner: WatchStream<T>, count: Arc<AtomicUsize>, notify: Arc<Notify>) -> Self {
        count.fetch_add(1, Ordering::Release);
        notify.notify_waiters();

        Self {
            inner,
            count,
            notify,
        }
    }
}

impl<T: Clone + Send + Sync + 'static> Stream for SubscribedStream<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.get_mut().inner).poll_next(cx)
    }
}

impl<T> Drop for SubscribedStream<T> {
    fn drop(&mut self) {
        self.count.fetch_sub(1, Ordering::Release);
        self.notify.notify_waiters();
    }
}
