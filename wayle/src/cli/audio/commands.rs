use clap::Subcommand;

/// Audio control subcommands.
#[derive(Subcommand, Debug)]
pub enum AudioCommands {
    /// Get or set output volume level
    #[command(allow_hyphen_values = true)]
    OutputVolume {
        /// Volume level (0-100) or relative adjustment (+5, -10)
        #[arg(value_name = "LEVEL")]
        level: Option<String>,
    },

    /// Toggle output mute state
    OutputMute,

    /// Get or set input volume level
    #[command(allow_hyphen_values = true)]
    InputVolume {
        /// Volume level (0-100) or relative adjustment (+5, -10)
        #[arg(value_name = "LEVEL")]
        level: Option<String>,
    },

    /// Toggle input mute state
    InputMute,

    /// List available audio sinks (outputs)
    Sinks,

    /// List available audio sources (inputs)
    Sources,

    /// Show current audio status
    Status,
}
