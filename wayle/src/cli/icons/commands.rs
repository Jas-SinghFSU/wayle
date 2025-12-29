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
        /// Source name (tabler, tabler-filled, simple-icons, lucide)
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
    /// Open the icons directory in file manager
    Open,
}

const INSTALL_HELP: &str = concat!(
    styled_header!("Sources:"),
    "\n",
    "    tabler         tb-   UI icons (home, settings, bell)      https://tabler.io/icons\n",
    "    tabler-filled  tbf-  Solid UI icons                       https://tabler.io/icons\n",
    "    simple-icons   si-   Brand logos (firefox, spotify)       https://simpleicons.org\n",
    "    lucide         ld-   Alternative UI icons                 https://lucide.dev/icons\n",
    "\n",
    styled_header!("Examples:"),
    "\n",
    "    wayle icons install tabler home settings bell\n",
    "        -> tb-home-symbolic, tb-settings-symbolic, tb-bell-symbolic\n",
    "\n",
    "    wayle icons install simple-icons firefox spotify\n",
    "        -> si-firefox-symbolic, si-spotify-symbolic\n",
    "\n",
    "Icons are saved to ~/.local/share/wayle/icons/ as GTK symbolic icons.",
);
