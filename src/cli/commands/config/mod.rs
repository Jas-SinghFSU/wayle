//! Configuration management commands.
mod get;
mod set;
mod watch;

use std::sync::Arc;

pub use get::GetCommand;
pub use set::SetCommand;
pub use watch::WatchCommand;

use crate::{cli::CommandRegistry, config_runtime::ConfigRuntime};

/// Registers all configuration-related commands with the command registry.
///
/// Registers commands in the "config" category for configuration management
/// operations like getting, setting, and watching configuration values.
///
/// # Arguments
///
/// * `registry` - Mutable reference to the command registry
/// * `config_runtime` - Shared configuration store for the commands
pub fn register_commands(registry: &mut CommandRegistry, config_runtime: Arc<ConfigRuntime>) {
    const CATEGORY_NAME: &str = "config";

    registry.register_command(
        CATEGORY_NAME,
        Box::new(GetCommand::new(Arc::clone(&config_runtime))),
    );

    registry.register_command(
        CATEGORY_NAME,
        Box::new(SetCommand::new(Arc::clone(&config_runtime))),
    );

    registry.register_command(
        CATEGORY_NAME,
        Box::new(WatchCommand::new(Arc::clone(&config_runtime))),
    );
}
