//! Ergonomic watcher utilities for Relm4 components.
//!
//! Provides the [`watch!`] macro for reactive stream watching with automatic
//! shutdown handling, stream merging, and error logging.

use std::pin::Pin;

use futures::stream::Stream;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::SubscribeChanges;

/// Converts a [`SubscribeChanges`] implementor into a stream.
///
/// This bridges the channel-based `subscribe_changes` API with the stream-based
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
/// Handler receives the command sender for full control. Use for conditional
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
