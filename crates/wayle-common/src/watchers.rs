//! Ergonomic watcher utilities for Relm4 components.
//!
//! Provides the [`watch!`] macro for reactive stream watching with automatic
//! shutdown handling, stream merging, and error logging.
//!
//! For watchers that need to be cancelled before component shutdown (e.g., when
//! a device changes), use [`watch_cancellable!`] with a [`CancellationToken`].
//!
//! [`CancellationToken`]: tokio_util::sync::CancellationToken

use std::pin::Pin;

use futures::stream::Stream;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_util::sync::CancellationToken;

use crate::SubscribeChanges;

/// Manages a cancellable watcher lifecycle.
///
/// Encapsulates the pattern of cancelling an existing watcher before spawning
/// a new one. Call [`reset`](Self::reset) to cancel any active watcher and
/// obtain a fresh token for the new watcher.
///
/// # Example
///
/// ```ignore
/// struct MyComponent {
///     device_watcher: WatcherToken,
/// }
///
/// // When device changes:
/// let token = self.device_watcher.reset();
/// Self::spawn_device_watchers(&sender, &device, token);
/// ```
#[derive(Debug, Default)]
pub struct WatcherToken(Option<CancellationToken>);

impl WatcherToken {
    /// Creates an empty watcher token with no active watcher.
    pub fn new() -> Self {
        Self(None)
    }

    /// Cancels any existing watcher and returns a fresh token.
    ///
    /// The returned token should be passed to `watch_cancellable!` or used
    /// directly with `token.cancelled()` in a `tokio::select!`.
    pub fn reset(&mut self) -> CancellationToken {
        if let Some(token) = self.0.take() {
            token.cancel();
        }
        let token = CancellationToken::new();
        self.0 = Some(token.clone());
        token
    }
}

/// Converts a [`SubscribeChanges`] implementor into a stream.
///
/// Bridges the channel-based `subscribe_changes` API with the stream-based
/// `watch!` macro. The returned stream is `'static` and does not borrow the
/// subscribable - it spawns internal watchers that send to the stream.
pub fn changes_stream<T: SubscribeChanges>(subscribable: &T) -> UnboundedReceiverStream<()> {
    let (tx, rx) = mpsc::unbounded_channel();
    subscribable.subscribe_changes(tx);
    UnboundedReceiverStream::new(rx)
}

/// Type alias for boxed streams used internally by the watch macro.
pub type BoxedStream = Pin<Box<dyn Stream<Item = ()> + Send>>;

/// Watches multiple streams and runs a handler when any emits.
///
/// Automatically handles:
/// - Stream pinning and type erasure
/// - Merging multiple streams with `select_all`
/// - Shutdown handling via Relm4's shutdown receiver
///
/// # Patterns
///
/// ## Auto-send
///
/// Handler returns `Result<T, E>`. On `Ok(value)`, sends `Cmd::Variant(value)`.
/// On `Err`, logs and continues.
///
/// ```ignore
/// watch!(sender, [streams], || fallible_handler() => Cmd::Variant);
/// ```
///
/// ## Manual
///
/// Handler receives the command sender for full control. Supports conditional
/// sends, multiple commands, or custom error handling.
///
/// ```ignore
/// watch!(sender, [streams], |out| {
///     if condition {
///         let _ = out.send(Cmd::A(value));
///     }
///     let _ = out.send(Cmd::B);
/// });
/// ```
///
/// # Examples
///
/// ```ignore
/// watch!(sender,
///     [changes_stream(&config.styling), wallpaper.watch_extraction()],
///     move || compile_css(&config) => ShellCmd::CssRecompiled
/// );
///
/// watch!(sender,
///     [audio.volume.watch()],
///     |out| {
///         let vol = audio.volume.get();
///         let _ = out.send(ShellCmd::VolumeChanged(vol));
///         if vol == 0.0 {
///             let _ = out.send(ShellCmd::Muted);
///         }
///     }
/// );
/// ```
#[macro_export]
macro_rules! watch {
    ($sender:expr, [$($stream:expr),* $(,)?], $handler:expr => $cmd:expr) => {{
        use ::futures::stream::StreamExt;
        use ::futures::stream::select_all;

        let streams: Vec<$crate::watchers::BoxedStream> = vec![
            $(
                Box::pin(StreamExt::map($stream, |_| ()))
                    as $crate::watchers::BoxedStream,
            )*
        ];

        let handler = $handler;
        let mapper = $cmd;

        $sender.command(move |out, shutdown| async move {
            let mut merged = select_all(streams);
            #[allow(unused_mut)]
            let mut handler = handler;

            ::tokio::select! {
                () = shutdown.wait() => {}
                () = async {
                    while merged.next().await.is_some() {
                        match handler() {
                            Ok(value) => {
                                let _ = out.send(mapper(value));
                            }
                            Err(err) => {
                                ::tracing::error!(error = %err, "Watcher handler failed");
                            }
                        }
                    }
                } => {}
            }
        });
    }};

    ($sender:expr, [$($stream:expr),* $(,)?], |$out:ident| $body:expr) => {{
        use ::futures::stream::StreamExt;
        use ::futures::stream::select_all;

        let streams: Vec<$crate::watchers::BoxedStream> = vec![
            $(
                Box::pin(StreamExt::map($stream, |_| ()))
                    as $crate::watchers::BoxedStream,
            )*
        ];

        $sender.command(move |$out, shutdown| async move {
            let mut merged = select_all(streams);

            ::tokio::select! {
                () = shutdown.wait() => {}
                () = async {
                    while merged.next().await.is_some() {
                        $body
                    }
                } => {}
            }
        });
    }};
}

/// Watches streams with cancellation support via [`CancellationToken`].
///
/// Unlike [`watch!`], this variant stops when either:
/// - The component shuts down (via Relm4's shutdown receiver)
/// - The provided cancellation token is cancelled
///
/// Use this for watchers tied to dynamic resources (e.g., audio devices,
/// media players) that may change during the component's lifetime.
///
/// # Example
///
/// ```ignore
/// struct MyComponent {
///     device_watcher_token: Option<CancellationToken>,
/// }
///
/// // When device changes:
/// if let Some(token) = self.device_watcher_token.take() {
///     token.cancel();
/// }
/// let token = CancellationToken::new();
/// self.device_watcher_token = Some(token.clone());
///
/// watch_cancellable!(sender, token, [device.volume.watch()], |out| {
///     let _ = out.send(Cmd::VolumeChanged);
/// });
/// ```
///
/// [`CancellationToken`]: tokio_util::sync::CancellationToken
#[macro_export]
macro_rules! watch_cancellable {
    ($sender:expr, $token:expr, [$($stream:expr),* $(,)?], |$out:ident| $body:expr) => {{
        use ::futures::stream::StreamExt;
        use ::futures::stream::select_all;

        let streams: Vec<$crate::watchers::BoxedStream> = vec![
            $(
                Box::pin(StreamExt::map($stream, |_| ()))
                    as $crate::watchers::BoxedStream,
            )*
        ];

        let token = $token;
        $sender.command(move |$out, shutdown| async move {
            let mut merged = select_all(streams);

            ::tokio::select! {
                () = shutdown.wait() => {}
                () = token.cancelled() => {}
                () = async {
                    while merged.next().await.is_some() {
                        $body
                    }
                } => {}
            }
        });
    }};
}
