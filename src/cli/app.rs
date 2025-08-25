use clap::{
    Parser, Subcommand,
    builder::styling::{AnsiColor, Effects, Styles},
};

use crate::cli::{config::ConfigCommands, media::MediaCommands, panel::PanelCommands};

fn get_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default().effects(Effects::BOLD))
        .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
        .literal(AnsiColor::Green.on_default().effects(Effects::BOLD))
        .placeholder(AnsiColor::Green.on_default())
        .error(AnsiColor::Red.on_default().effects(Effects::BOLD))
        .valid(AnsiColor::Green.on_default().effects(Effects::BOLD))
        .invalid(AnsiColor::Yellow.on_default().effects(Effects::BOLD))
}

/// Wayle - A Wayland compositor agnostic shell
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(styles = get_styles())]
pub struct Cli {
    /// The command to execute.
    #[command(subcommand)]
    pub command: Commands,
}

/// Available CLI commands.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Panel management commands
    Panel {
        /// Panel subcommand to execute.
        #[command(subcommand)]
        command: PanelCommands,
    },
    /// Configuration management commands
    Config {
        /// Configuration subcommand to execute.
        #[command(subcommand)]
        command: ConfigCommands,
    },
    /// Media player control commands
    Media {
        /// Media subcommand to execute.
        #[command(subcommand)]
        command: MediaCommands,
    },
}
