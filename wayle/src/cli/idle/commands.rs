use clap::Subcommand;

/// Idle inhibit control subcommands.
#[derive(Subcommand, Debug)]
pub enum IdleCommands {
    /// Enable idle inhibition
    On {
        /// Duration in minutes (omit to use default duration)
        #[arg(value_name = "MINUTES")]
        minutes: Option<u32>,

        /// Force indefinite mode (ignore default duration)
        #[arg(long, short = 'i')]
        indefinite: bool,
    },

    /// Disable idle inhibition
    Off,

    /// Adjust timer duration (upper limit)
    #[command(allow_hyphen_values = true)]
    Duration {
        /// +N to add, -N to subtract, N to set absolute
        #[arg(value_name = "VALUE")]
        value: String,
    },

    /// Adjust remaining time on active timer
    #[command(allow_hyphen_values = true)]
    Remaining {
        /// +N to add, -N to subtract, N to set absolute
        #[arg(value_name = "VALUE")]
        value: String,
    },

    /// Show current idle inhibit status
    Status,

    /// Toggle idle inhibition on/off
    Toggle {
        /// Use indefinite mode when enabling
        #[arg(long, short = 'i')]
        indefinite: bool,
    },
}
