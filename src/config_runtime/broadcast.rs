use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use tokio::{
    sync::mpsc::{self, Receiver, Sender},
    task::JoinHandle,
};

use crate::config_runtime::{
    changes::{ConfigChange, ConfigError},
    path_ops::path_matches,
};

/// Commands sent to the broadcast actor thread
pub enum BroadcastCommand {
    /// Subscribe to configuration changes matching a pattern
    Subscribe {
        /// Unique subscription identifier
        id: usize,
        /// Pattern to match against config paths
        pattern: String,
        /// Channel sender for config change notifications
        sender: Sender<ConfigChange>,
    },
    /// Remove a subscription by ID
    Unsubscribe {
        /// Subscription ID to remove
        id: usize,
    },
    /// Broadcast a configuration change to all matching subscribers
    Broadcast(ConfigChange),
}

/// Internal subscription data stored in the actor
struct ActorSubscription {
    id: usize,
    pattern: String,
    sender: Sender<ConfigChange>,
}

/// A subscription handle that automatically cleans up when dropped.
///
/// This handle uses RAII to ensure subscriptions are properly cleaned up
/// when UI components are removed or go out of scope.
pub struct Subscription {
    id: usize,
    service: BroadcastService,
    receiver: Receiver<ConfigChange>,
}

/// Handle to the broadcast service
///
/// This is the main interface for interacting with the configuration broadcast system.
/// It uses an actor pattern where a dedicated task owns all subscriber state and
/// processes commands via message passing.
#[derive(Clone)]
pub struct BroadcastService {
    command_tx: Sender<BroadcastCommand>,
    next_id: Arc<AtomicUsize>,
    _handle: Arc<JoinHandle<()>>,
}

impl Default for BroadcastService {
    fn default() -> Self {
        Self::new()
    }
}

impl BroadcastService {
    /// Creates a new broadcast service with its own dedicated actor task.
    ///
    /// The actor task will run until the service is dropped or explicitly shutdown.
    pub fn new() -> Self {
        let (command_tx, mut command_rx) = mpsc::channel(100);

        let handle = tokio::spawn(async move {
            Self::broadcast_actor_loop(&mut command_rx).await;
        });

        Self {
            command_tx,
            next_id: Arc::new(AtomicUsize::new(1)),
            _handle: Arc::new(handle),
        }
    }

    /// Subscribe to configuration changes matching the given pattern.
    ///
    /// Returns a subscription handle that includes the receiver for changes.
    /// The subscription will automatically clean up when the handle is dropped.
    /// Pattern matching supports wildcards like "modules.clock.*" or "modules.*".
    ///
    /// # Arguments
    /// * `pattern` - Pattern to match configuration paths (supports "*" wildcards)
    ///
    /// # Errors
    /// Returns `ConfigError::ServiceUnavailable` if the broadcast service is not running.
    pub async fn subscribe(&self, pattern: &str) -> Result<Subscription, ConfigError> {
        let (tx, rx) = mpsc::channel(100);
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);

        self.command_tx
            .send(BroadcastCommand::Subscribe {
                id,
                pattern: String::from(pattern),
                sender: tx,
            })
            .await
            .map_err(|_| ConfigError::ServiceUnavailable {
                service: String::from("broadcast"),
                details: String::from("Broadcast service is not running"),
            })?;

        Ok(Subscription {
            id,
            service: self.clone(),
            receiver: rx,
        })
    }

    /// Broadcast a configuration change to all matching subscribers.
    ///
    /// The change will be filtered and sent only to subscribers whose patterns match
    /// the change path. This is more efficient than broadcasting to all subscribers.
    ///
    /// # Arguments
    /// * `change` - The configuration change to broadcast
    ///
    /// # Errors
    /// Returns `ConfigError::ServiceUnavailable` if the broadcast service is not running.
    pub async fn broadcast(&self, change: ConfigChange) -> Result<(), ConfigError> {
        self.command_tx
            .send(BroadcastCommand::Broadcast(change))
            .await
            .map_err(|_| ConfigError::ServiceUnavailable {
                service: String::from("broadcast"),
                details: String::from("Broadcast service is not running"),
            })
    }

    async fn broadcast_actor_loop(command_rx: &mut Receiver<BroadcastCommand>) {
        let mut subscriptions = Vec::new();

        while let Some(command) = command_rx.recv().await {
            match command {
                BroadcastCommand::Subscribe {
                    id,
                    pattern,
                    sender,
                } => {
                    subscriptions.push(ActorSubscription {
                        id,
                        pattern,
                        sender,
                    });
                }

                BroadcastCommand::Unsubscribe { id } => {
                    subscriptions.retain(|sub| sub.id != id);
                }

                BroadcastCommand::Broadcast(change) => {
                    subscriptions.retain(|sub| {
                        if path_matches(&change.path, &sub.pattern) {
                            sub.sender.try_send(change.clone()).is_ok()
                        } else {
                            true
                        }
                    });
                }
            }
        }
    }
}

impl Subscription {
    /// Get the receiver for configuration changes.
    ///
    /// This receiver will only receive changes that match the subscription pattern.
    pub fn receiver(&self) -> &Receiver<ConfigChange> {
        &self.receiver
    }

    /// Get a mutable reference to the receiver for configuration changes.
    ///
    /// This is needed for async receiving operations.
    pub fn receiver_mut(&mut self) -> &mut Receiver<ConfigChange> {
        &mut self.receiver
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        let _ = self
            .service
            .command_tx
            .try_send(BroadcastCommand::Unsubscribe { id: self.id });
    }
}
