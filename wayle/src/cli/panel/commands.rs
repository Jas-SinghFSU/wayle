use clap::Subcommand;

/// Panel management subcommands.
#[derive(Subcommand, Debug)]
pub enum PanelCommands {
    /// Start the panel daemon
    Start,

    /// Stop the panel daemon
    Stop,

    /// Restart the panel daemon
    Restart,

    /// Check panel status
    Status,

    /// Open panel settings
    Settings,

    /// Open GTK Inspector for debugging
    Inspect,

    /// Hide the bar on a monitor
    Hide {
        /// Monitor connector name (e.g., "DP-1"). Omit to hide all.
        monitor: Option<String>,
    },

    /// Show the bar on a monitor
    Show {
        /// Monitor connector name (e.g., "DP-1"). Omit to show all.
        monitor: Option<String>,
    },

    /// Toggle bar visibility on a monitor
    Toggle {
        /// Monitor connector name (e.g., "DP-1"). Omit to toggle all.
        monitor: Option<String>,
    },
}
