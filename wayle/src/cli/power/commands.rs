use clap::Subcommand;

/// Power profile control subcommands.
#[derive(Subcommand, Debug)]
pub enum PowerCommands {
    /// Show current power profile
    Status,

    /// Set power profile
    Set {
        /// Profile name (power-saver, balanced, performance)
        #[arg(value_name = "PROFILE")]
        profile: String,
    },

    /// Cycle to next power profile
    Cycle,

    /// List available power profiles
    List,
}
