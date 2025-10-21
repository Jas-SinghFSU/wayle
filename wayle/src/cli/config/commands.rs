use clap::Subcommand;

/// Configuration management subcommands.
#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Get the value of a configuration path
    Get {
        /// The configuration path to retrieve (e.g., "modules.battery.enabled")
        path: String,
    },
    /// Set the value of a configuration path
    Set {
        /// The configuration path to set (e.g., "modules.battery.enabled")
        path: String,
        /// The value to set (use JSON format for complex types)
        value: String,
    },
}
