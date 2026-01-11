use clap::Subcommand;

/// System tray control subcommands.
#[derive(Subcommand, Debug)]
pub enum SystrayCommands {
    /// List all system tray items
    List,

    /// Activate a tray item by ID
    Activate {
        /// Tray item ID to activate
        #[arg(value_name = "ID")]
        id: String,
    },

    /// Show system tray status
    Status,
}
