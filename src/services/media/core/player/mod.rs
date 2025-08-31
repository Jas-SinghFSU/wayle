pub(crate) mod monitoring;

use std::{sync::Arc, time::Duration};

use futures::Stream;
use monitoring::PlayerMonitor;
use tokio_util::sync::CancellationToken;
use zbus::{
    Connection,
    fdo::PropertiesProxy,
    names::{InterfaceName, MemberName, OwnedBusName},
    zvariant::ObjectPath,
};

use crate::{
    services::{
        common::{NULL_PATH, Property},
        media::{
            MediaError,
            core::metadata::TrackMetadata,
            proxy::{MediaPlayer2PlayerProxy, MediaPlayer2Proxy},
            types::{LoopMode, PlaybackState, PlayerId, ShuffleMode, Volume},
        },
    },
    unwrap_bool, unwrap_string_or, watch_all,
};

/// Reactive player model with fine-grained property updates.
///
/// Each property can be watched independently for efficient UI updates.
/// Properties are updated by the D-Bus monitoring layer.
#[derive(Clone, Debug)]
pub struct Player {
    /// D-Bus proxy for controlling this player
    proxy: MediaPlayer2PlayerProxy<'static>,

    /// Unique identifier for this player instance
    pub id: PlayerId,
    /// Human-readable name of the player application
    pub identity: Property<String>,
    /// Desktop file name for the player application
    pub desktop_entry: Property<Option<String>>,

    /// Current playback state (Playing, Paused, Stopped)
    pub playback_state: Property<PlaybackState>,
    /// Current loop mode (None, Track, Playlist)
    pub loop_mode: Property<LoopMode>,
    /// Current shuffle mode (On, Off, Unsupported)
    pub shuffle_mode: Property<ShuffleMode>,
    /// Current volume level
    pub volume: Property<Volume>,

    /// Current track metadata
    pub metadata: Arc<TrackMetadata>,

    /// Whether the player can be controlled
    pub can_control: Property<bool>,
    /// Whether playback can be started
    pub can_play: Property<bool>,
    /// Whether the player can skip to the next track
    pub can_go_next: Property<bool>,
    /// Whether the player can go to the previous track
    pub can_go_previous: Property<bool>,
    /// Whether the player supports seeking
    pub can_seek: Property<bool>,
    /// Whether the player supports loop modes
    pub can_loop: Property<bool>,
    /// Whether the player supports shuffle
    pub can_shuffle: Property<bool>,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Player {
    fn new(
        id: PlayerId,
        identity: String,
        proxy: MediaPlayer2PlayerProxy<'static>,
        metadata: Arc<TrackMetadata>,
    ) -> Self {
        Self {
            proxy,
            id,
            identity: Property::new(identity),
            desktop_entry: Property::new(None),

            playback_state: Property::new(PlaybackState::Stopped),
            loop_mode: Property::new(LoopMode::None),
            shuffle_mode: Property::new(ShuffleMode::Off),
            volume: Property::new(Volume::default()),

            metadata,

            can_control: Property::new(false),
            can_play: Property::new(false),
            can_go_next: Property::new(false),
            can_go_previous: Property::new(false),
            can_seek: Property::new(false),
            can_loop: Property::new(false),
            can_shuffle: Property::new(false),
        }
    }

    pub(crate) async fn get(
        connection: &Connection,
        player_id: PlayerId,
    ) -> Result<Arc<Self>, MediaError> {
        let bus_name = OwnedBusName::try_from(player_id.bus_name())
            .map_err(|e| MediaError::InitializationFailed(format!("Invalid bus name: {e}")))?;

        let base_proxy = MediaPlayer2Proxy::builder(connection)
            .destination(bus_name.clone())
            .map_err(MediaError::DbusError)?
            .build()
            .await
            .map_err(MediaError::DbusError)?;

        let player_proxy = MediaPlayer2PlayerProxy::builder(connection)
            .destination(bus_name)
            .map_err(MediaError::DbusError)?
            .build()
            .await
            .map_err(MediaError::DbusError)?;

        let identity = unwrap_string_or!(
            base_proxy.identity().await,
            player_id.bus_name().to_string()
        );
        let desktop_entry = base_proxy.desktop_entry().await.ok();

        let metadata = TrackMetadata::get(&player_proxy).await;
        let player = Self::new(player_id, identity, player_proxy.clone(), metadata);
        player.desktop_entry.set(desktop_entry);

        Self::refresh_properties(&player, &player_proxy).await;

        Ok(Arc::new(player))
    }

    pub(crate) async fn get_live(
        connection: &Connection,
        player_id: PlayerId,
        cancellation_token: CancellationToken,
    ) -> Result<Arc<Self>, MediaError> {
        let bus_name = OwnedBusName::try_from(player_id.bus_name())
            .map_err(|e| MediaError::InitializationFailed(format!("Invalid bus name: {e}")))?;

        let base_proxy = MediaPlayer2Proxy::builder(connection)
            .destination(bus_name.clone())
            .map_err(MediaError::DbusError)?
            .build()
            .await
            .map_err(MediaError::DbusError)?;

        let player_proxy = MediaPlayer2PlayerProxy::builder(connection)
            .destination(bus_name)
            .map_err(MediaError::DbusError)?
            .build()
            .await
            .map_err(MediaError::DbusError)?;

        let identity = unwrap_string_or!(
            base_proxy.identity().await,
            player_id.bus_name().to_string()
        );
        let desktop_entry = base_proxy.desktop_entry().await.ok();

        let metadata =
            TrackMetadata::get_live(player_proxy.clone(), cancellation_token.child_token()).await;
        let player = Self::new(player_id.clone(), identity, player_proxy.clone(), metadata);
        player.desktop_entry.set(desktop_entry);

        Self::refresh_properties(&player, &player_proxy).await;

        let player = Arc::new(player);
        PlayerMonitor::start(
            player_id,
            Arc::clone(&player),
            player_proxy,
            cancellation_token,
        );

        Ok(player)
    }

    async fn refresh_properties(player: &Player, proxy: &MediaPlayer2PlayerProxy<'_>) {
        if let Ok(status) = proxy.playback_status().await {
            player
                .playback_state
                .set(PlaybackState::from(status.as_str()));
        }

        if let Ok(loop_status) = proxy.loop_status().await {
            player.loop_mode.set(LoopMode::from(loop_status.as_str()));
        }

        if let Ok(shuffle) = proxy.shuffle().await {
            player.shuffle_mode.set(ShuffleMode::from(shuffle));
        }

        if let Ok(volume) = proxy.volume().await {
            player.volume.set(Volume::from(volume));
        }

        let can_control = unwrap_bool!(proxy.can_control().await);
        let can_play = unwrap_bool!(proxy.can_play().await);
        let can_go_next = unwrap_bool!(proxy.can_go_next().await);
        let can_go_previous = unwrap_bool!(proxy.can_go_previous().await);
        let can_seek = unwrap_bool!(proxy.can_seek().await);
        let can_loop = proxy.loop_status().await.is_ok();
        let can_shuffle = proxy.shuffle().await.is_ok();

        player.update_capabilities(
            can_control,
            can_play,
            can_go_next,
            can_go_previous,
            can_seek,
            can_loop,
            can_shuffle,
        );
    }

    /// Play or pause playback.
    ///
    /// # Errors
    ///
    /// Returns `MediaError::ControlFailed` if the D-Bus operation fails
    pub async fn play_pause(&self) -> Result<(), MediaError> {
        self.proxy
            .play_pause()
            .await
            .map_err(|e| MediaError::ControlFailed(format!("Play/pause failed: {e}")))?;
        Ok(())
    }

    /// Skip to next track.
    ///
    /// # Errors
    ///
    /// Returns `MediaError::ControlFailed` if the D-Bus operation fails
    pub async fn next(&self) -> Result<(), MediaError> {
        self.proxy
            .next()
            .await
            .map_err(|e| MediaError::ControlFailed(format!("Next failed: {e}")))?;
        Ok(())
    }

    /// Go to previous track.
    ///
    /// # Errors
    ///
    /// Returns `MediaError::ControlFailed` if the D-Bus operation fails
    pub async fn previous(&self) -> Result<(), MediaError> {
        self.proxy
            .previous()
            .await
            .map_err(|e| MediaError::ControlFailed(format!("Previous failed: {e}")))?;
        Ok(())
    }

    /// Seek by offset (relative position change).
    ///
    /// # Errors
    ///
    /// Returns `MediaError::ControlFailed` if the D-Bus operation fails
    pub async fn seek(&self, offset: Duration) -> Result<(), MediaError> {
        let offset_micros = offset.as_micros() as i64;
        self.proxy
            .seek(offset_micros)
            .await
            .map_err(|e| MediaError::ControlFailed(format!("Seek failed: {e}")))?;
        Ok(())
    }

    /// Set position to an absolute value.
    ///
    /// # Errors
    ///
    /// Returns `MediaError::ControlFailed` if the D-Bus operation fails
    pub async fn set_position(&self, position: Duration) -> Result<(), MediaError> {
        let track_id = self.metadata.track_id.get();
        let track_path = track_id.as_deref().unwrap_or(NULL_PATH);
        let track_object_path = ObjectPath::try_from(track_path)
            .map_err(|e| MediaError::ControlFailed(format!("Invalid track ID: {e}")))?;

        let position_micros = position.as_micros() as i64;
        self.proxy
            .set_position(&track_object_path, position_micros)
            .await
            .map_err(|e| MediaError::ControlFailed(format!("Set position failed: {e}")))?;
        Ok(())
    }

    /// Get current playback position.
    ///
    /// # Errors
    ///
    /// Returns error if the D-Bus operation fails
    pub async fn position(&self) -> Result<Duration, MediaError> {
        let connection = Connection::session().await.map_err(MediaError::DbusError)?;
        let destination = self.proxy.inner().destination().to_owned();
        let path = self.proxy.inner().path().to_owned();

        let proxy = PropertiesProxy::builder(&connection)
            .destination(destination)
            .map_err(|e| {
                MediaError::ControlFailed(format!("Failed to create properties proxy: {e}"))
            })?
            .path(path)
            .map_err(|e| MediaError::ControlFailed(format!("Failed to set path: {e}")))?
            .build()
            .await
            .map_err(|e| {
                MediaError::ControlFailed(format!("Failed to build properties proxy: {e}"))
            })?;

        let interface = InterfaceName::try_from("org.mpris.MediaPlayer2.Player")
            .map_err(|e| MediaError::ControlFailed(format!("Invalid interface name: {e}")))?;
        let property = MemberName::try_from("Position")
            .map_err(|e| MediaError::ControlFailed(format!("Invalid property name: {e}")))?;

        let value = proxy
            .get(interface, &property)
            .await
            .map_err(|e| MediaError::ControlFailed(format!("Failed to get position: {e}")))?;

        let micros = i64::try_from(&value)
            .map_err(|e| MediaError::ControlFailed(format!("Failed to parse position: {e}")))?;

        Ok(Duration::from_micros(micros.max(0) as u64))
    }

    /// Watch position changes for this player.
    ///
    /// Polls position every second. Use `watch_position_with_interval` for custom intervals.
    pub fn watch_position(&self) -> impl Stream<Item = Duration> + Send {
        self.watch_position_with_interval(Duration::from_secs(1))
    }

    /// Watch position changes with a specified polling interval.
    ///
    /// Returns a stream that emits the current position at the specified interval.
    /// Only emits when position actually changes to avoid redundant updates.
    pub fn watch_position_with_interval(
        &self,
        interval: Duration,
    ) -> impl Stream<Item = Duration> + Send {
        let player = self.clone();
        async_stream::stream! {
            let mut last_position: Option<Duration> = None;

            loop {
                match player.position().await {
                    Ok(position) => {
                        if last_position != Some(position) {
                            last_position = Some(position);
                            yield position;
                        }
                    }
                    Err(_) => break,
                }
                tokio::time::sleep(interval).await;
            }
        }
    }

    /// Set loop mode.
    ///
    /// # Errors
    ///
    /// Returns `MediaError::ControlFailed` if the D-Bus operation fails,
    /// or if the loop mode is unsupported
    pub async fn set_loop_mode(&self, mode: LoopMode) -> Result<(), MediaError> {
        let status = match mode {
            LoopMode::None => "None",
            LoopMode::Track => "Track",
            LoopMode::Playlist => "Playlist",
            LoopMode::Unsupported => {
                return Err(MediaError::ControlFailed(
                    "Loop mode not supported".to_string(),
                ));
            }
        };

        self.proxy
            .set_loop_status(status)
            .await
            .map_err(|e| MediaError::ControlFailed(format!("Set loop mode failed: {e}")))?;
        Ok(())
    }

    /// Set shuffle mode.
    ///
    /// # Errors
    ///
    /// Returns `MediaError::ControlFailed` if the D-Bus operation fails,
    /// or if shuffle is unsupported
    pub async fn set_shuffle_mode(&self, mode: ShuffleMode) -> Result<(), MediaError> {
        let shuffle = match mode {
            ShuffleMode::On => true,
            ShuffleMode::Off => false,
            ShuffleMode::Unsupported => {
                return Err(MediaError::ControlFailed(
                    "Shuffle not supported".to_string(),
                ));
            }
        };

        self.proxy
            .set_shuffle(shuffle)
            .await
            .map_err(|e| MediaError::ControlFailed(format!("Set shuffle failed: {e}")))?;
        Ok(())
    }

    /// Set volume.
    ///
    /// # Errors
    ///
    /// Returns `MediaError::ControlFailed` if the D-Bus operation fails
    pub async fn set_volume(&self, volume: Volume) -> Result<(), MediaError> {
        self.proxy
            .set_volume(*volume)
            .await
            .map_err(|e| MediaError::ControlFailed(format!("Set volume failed: {e}")))?;
        Ok(())
    }

    /// Toggle loop mode to the next state.
    ///
    /// Cycles through: None → Track → Playlist → None
    ///
    /// # Errors
    ///
    /// Returns `MediaError::OperationNotSupported` if loop mode is unsupported
    pub async fn toggle_loop(&self) -> Result<(), MediaError> {
        let current = self.loop_mode.get();
        let next = match current {
            LoopMode::None => LoopMode::Track,
            LoopMode::Track => LoopMode::Playlist,
            LoopMode::Playlist => LoopMode::None,
            LoopMode::Unsupported => {
                return Err(MediaError::OperationNotSupported(
                    "Loop mode not supported".to_string(),
                ));
            }
        };

        self.set_loop_mode(next).await
    }

    /// Toggle shuffle mode between on and off.
    ///
    /// # Errors
    ///
    /// Returns `MediaError::OperationNotSupported` if shuffle is unsupported
    pub async fn toggle_shuffle(&self) -> Result<(), MediaError> {
        let current = self.shuffle_mode.get();
        let next = match current {
            ShuffleMode::Off => ShuffleMode::On,
            ShuffleMode::On => ShuffleMode::Off,
            ShuffleMode::Unsupported => {
                return Err(MediaError::OperationNotSupported(
                    "Shuffle not supported".to_string(),
                ));
            }
        };

        self.set_shuffle_mode(next).await
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn update_capabilities(
        &self,
        can_control: bool,
        can_play: bool,
        can_go_next: bool,
        can_go_previous: bool,
        can_seek: bool,
        can_loop: bool,
        can_shuffle: bool,
    ) {
        self.can_control.set(can_control);
        self.can_play.set(can_play);
        self.can_go_next.set(can_go_next);
        self.can_go_previous.set(can_go_previous);
        self.can_seek.set(can_seek);
        self.can_loop.set(can_loop);
        self.can_shuffle.set(can_shuffle);
    }

    /// Watch for any player property changes.
    ///
    /// Returns a stream that emits a clone of the player whenever any property changes,
    /// including metadata, playback state, capabilities, or any other tracked field.
    pub fn watch(&self) -> impl Stream<Item = Player> + Send {
        watch_all!(
            self,
            identity,
            desktop_entry,
            playback_state,
            loop_mode,
            shuffle_mode,
            volume,
            metadata,
            can_control,
            can_play,
            can_go_next,
            can_go_previous,
            can_seek,
            can_loop,
            can_shuffle
        )
    }
}
