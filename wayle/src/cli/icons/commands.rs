use clap::Subcommand;

use crate::styled_header;

/// Icon management subcommands.
#[derive(Subcommand, Debug)]
pub enum IconsCommands {
    /// Install bundled icons required by Wayle components
    Setup,
    /// Install icons from a CDN source
    #[command(after_long_help = INSTALL_HELP)]
    Install {
        /// Source name (run 'wayle icons sources' to see available sources)
        source: String,
        /// Icon slugs to install (e.g., home settings bell)
        #[arg(required = true)]
        slugs: Vec<String>,
    },
    /// Remove installed icons
    Remove {
        /// Icon names to remove (e.g., tb-home-symbolic si-firefox-symbolic)
        #[arg(required = true)]
        names: Vec<String>,
    },
    /// List available icon sources
    Sources,
    /// List installed icons
    List {
        /// Filter by source prefix (e.g., tb, si, md)
        #[arg(short, long)]
        source: Option<String>,
        /// Interactive fuzzy search (requires fzf)
        #[arg(short, long)]
        interactive: bool,
    },
    /// Open the icons directory in file manager
    Open,
}

const INSTALL_HELP: &str = concat!(
    styled_header!("Examples:"),
    "\n",
    "    wayle icons install tabler home settings bell\n",
    "        -> tb-home-symbolic, tb-settings-symbolic, tb-bell-symbolic\n",
    "\n",
    "    wayle icons install simple-icons firefox spotify\n",
    "        -> si-firefox-symbolic, si-spotify-symbolic\n",
    "\n",
    "Run 'wayle icons sources' to see all available icon sources.\n",
    "Icons are saved to ~/.local/share/wayle/icons/ as GTK symbolic icons.",
);
