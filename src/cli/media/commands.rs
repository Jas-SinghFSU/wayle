use clap::{Subcommand, ValueEnum};

/// Media player control subcommands.
#[derive(Subcommand, Debug)]
pub enum MediaCommands {
    /// List all available media players
    List,

    /// Toggle play/pause for a media player
    PlayPause {
        /// Player identifier (number or partial name match)
        #[arg(value_name = "PLAYER_ID")]
        player: Option<String>,
    },

    /// Skip to next track
    Next {
        /// Player identifier (number or partial name match)
        #[arg(value_name = "PLAYER_ID")]
        player: Option<String>,
    },

    /// Go to previous track
    Previous {
        /// Player identifier (number or partial name match)
        #[arg(value_name = "PLAYER_ID")]
        player: Option<String>,
    },

    /// Seek to a position in the current track
    Seek {
        /// Target position (seconds, mm:ss, percentage%, or relative +/-seconds)
        position: String,
        /// Player identifier (number or partial name match)
        #[arg(value_name = "PLAYER_ID")]
        player: Option<String>,
    },

    /// Toggle or set shuffle mode
    Shuffle {
        /// Shuffle state
        #[arg(value_name = "SHUFFLE_STATE")]
        state: Option<ShuffleModeArg>,
        /// Player identifier (number or partial name match)
        #[arg(value_name = "PLAYER_ID")]
        player: Option<String>,
    },

    /// Set loop/repeat mode
    #[command(name = "loop")]
    Loop {
        /// Loop mode
        mode: LoopModeArg,
        /// Player identifier (number or partial name match)
        #[arg(value_name = "PLAYER_ID")]
        player: Option<String>,
    },

    /// Get or set the active media player
    Active {
        /// Player to set as active (number or partial name match)
        #[arg(value_name = "PLAYER_ID")]
        player: Option<String>,
    },

    /// Display detailed information about a media player
    Info {
        /// Player identifier (number or partial name match)
        #[arg(value_name = "PLAYER_ID")]
        player: Option<String>,
    },
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum LoopModeArg {
    /// No looping
    None,
    /// Loop current track
    Track,
    /// Loop entire playlist
    Playlist,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum ShuffleModeArg {
    /// Enable shuffle
    On,
    /// Disable shuffle
    Off,
    /// Toggle shuffle state
    Toggle,
}
