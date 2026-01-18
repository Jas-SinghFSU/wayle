use super::Palette;
use crate::schemas::styling::ThemeEntry;

/// Catppuccin Mocha color constants.
#[allow(missing_docs)]
pub mod catppuccin_mocha {
    pub const BG: &str = "#11111b";
    pub const SURFACE: &str = "#181825";
    pub const ELEVATED: &str = "#1e1e2e";
    pub const FG: &str = "#cdd6f4";
    pub const FG_MUTED: &str = "#bac2de";
    pub const PRIMARY: &str = "#b4befe";
    pub const RED: &str = "#f38ba8";
    pub const YELLOW: &str = "#f9e2af";
    pub const GREEN: &str = "#a6e3a1";
    pub const BLUE: &str = "#74c7ec";
}

/// All built-in theme entries.
pub fn builtins() -> Vec<ThemeEntry> {
    vec![
        ThemeEntry {
            name: String::from("catppuccin"),
            palette: catppuccin(),
            builtin: true,
        },
        ThemeEntry {
            name: String::from("catppuccin-latte"),
            palette: catppuccin_latte(),
            builtin: true,
        },
        ThemeEntry {
            name: String::from("gruvbox"),
            palette: gruvbox(),
            builtin: true,
        },
        ThemeEntry {
            name: String::from("tokyo-night"),
            palette: tokyo_night(),
            builtin: true,
        },
        ThemeEntry {
            name: String::from("rose-pine"),
            palette: rose_pine(),
            builtin: true,
        },
        ThemeEntry {
            name: String::from("dracula"),
            palette: dracula(),
            builtin: true,
        },
        ThemeEntry {
            name: String::from("nord"),
            palette: nord(),
            builtin: true,
        },
        ThemeEntry {
            name: String::from("everforest"),
            palette: everforest(),
            builtin: true,
        },
    ]
}

/// Default palette (Catppuccin Mocha).
pub fn catppuccin() -> Palette {
    use catppuccin_mocha::*;
    Palette {
        bg: BG.to_owned(),
        surface: SURFACE.to_owned(),
        elevated: ELEVATED.to_owned(),
        fg: FG.to_owned(),
        fg_muted: FG_MUTED.to_owned(),
        primary: PRIMARY.to_owned(),
        red: RED.to_owned(),
        yellow: YELLOW.to_owned(),
        green: GREEN.to_owned(),
        blue: BLUE.to_owned(),
    }
}

fn catppuccin_latte() -> Palette {
    Palette {
        bg: String::from("#eff1f5"),
        surface: String::from("#e6e9ef"),
        elevated: String::from("#dce0e8"),
        fg: String::from("#4c4f69"),
        fg_muted: String::from("#6c6f85"),
        primary: String::from("#7287fd"),
        red: String::from("#d20f39"),
        yellow: String::from("#df8e1d"),
        green: String::from("#40a02b"),
        blue: String::from("#1e66f5"),
    }
}

fn gruvbox() -> Palette {
    Palette {
        bg: String::from("#282828"),
        surface: String::from("#3c3836"),
        elevated: String::from("#504945"),
        fg: String::from("#ebdbb2"),
        fg_muted: String::from("#d5c4a1"),
        primary: String::from("#83a598"),
        red: String::from("#fb4934"),
        yellow: String::from("#fabd2f"),
        green: String::from("#b8bb26"),
        blue: String::from("#8ec07c"),
    }
}

fn tokyo_night() -> Palette {
    Palette {
        bg: String::from("#16161e"),
        surface: String::from("#1a1b26"),
        elevated: String::from("#202230"),
        fg: String::from("#c0caf5"),
        fg_muted: String::from("#a9b1d6"),
        primary: String::from("#7aa2f7"),
        red: String::from("#f7768e"),
        yellow: String::from("#e0af68"),
        green: String::from("#9ece6a"),
        blue: String::from("#7dcfff"),
    }
}

fn rose_pine() -> Palette {
    Palette {
        bg: String::from("#191724"),
        surface: String::from("#1f1d2e"),
        elevated: String::from("#26233a"),
        fg: String::from("#e0def4"),
        fg_muted: String::from("#908caa"),
        primary: String::from("#c4a7e7"),
        red: String::from("#eb6f92"),
        yellow: String::from("#f6c177"),
        green: String::from("#31748f"),
        blue: String::from("#9ccfd8"),
    }
}

fn dracula() -> Palette {
    Palette {
        bg: String::from("#282a36"),
        surface: String::from("#343746"),
        elevated: String::from("#44475a"),
        fg: String::from("#f8f8f2"),
        fg_muted: String::from("#6272a4"),
        primary: String::from("#bd93f9"),
        red: String::from("#ff5555"),
        yellow: String::from("#f1fa8c"),
        green: String::from("#50fa7b"),
        blue: String::from("#8be9fd"),
    }
}

fn nord() -> Palette {
    Palette {
        bg: String::from("#2e3440"),
        surface: String::from("#3b4252"),
        elevated: String::from("#434c5e"),
        fg: String::from("#eceff4"),
        fg_muted: String::from("#d8dee9"),
        primary: String::from("#88c0d0"),
        red: String::from("#bf616a"),
        yellow: String::from("#ebcb8b"),
        green: String::from("#a3be8c"),
        blue: String::from("#81a1c1"),
    }
}

fn everforest() -> Palette {
    Palette {
        bg: String::from("#2d353b"),
        surface: String::from("#343f44"),
        elevated: String::from("#3d484d"),
        fg: String::from("#d3c6aa"),
        fg_muted: String::from("#9da9a0"),
        primary: String::from("#7fbbb3"),
        red: String::from("#e67e80"),
        yellow: String::from("#dbbc7f"),
        green: String::from("#a7c080"),
        blue: String::from("#83c092"),
    }
}
