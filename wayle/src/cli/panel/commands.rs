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
}
