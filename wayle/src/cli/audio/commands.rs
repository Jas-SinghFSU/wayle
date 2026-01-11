use clap::Subcommand;

/// Audio control subcommands.
#[derive(Subcommand, Debug)]
pub enum AudioCommands {
    /// Get or set volume level
    #[command(allow_hyphen_values = true)]
    Volume {
        /// Volume level (0-100) or relative adjustment (+5, -10)
        #[arg(value_name = "LEVEL")]
        level: Option<String>,
    },

    /// Toggle mute state
    Mute,

    /// List available audio sinks (outputs)
    Sinks,

    /// List available audio sources (inputs)
    Sources,

    /// Show current audio status
    Status,
}
