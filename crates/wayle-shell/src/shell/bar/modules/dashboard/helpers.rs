use std::fs;

use gdk4::Display;
use gtk4::IconTheme;

const FALLBACK_ICON: &str = "cm-wayle-symbolic";

struct DistroInfo {
    id: String,
    logo: Option<String>,
}

fn detect_distro() -> Option<DistroInfo> {
    let content = fs::read_to_string("/etc/os-release")
        .or_else(|_| fs::read_to_string("/usr/lib/os-release"))
        .ok()?;

    let mut id = None;
    let mut logo = None;

    for line in content.lines() {
        let line = line.trim();
        if let Some(value) = line.strip_prefix("ID=") {
            id = Some(unquote(value));
        } else if let Some(value) = line.strip_prefix("LOGO=") {
            logo = Some(unquote(value));
        }
    }

    Some(DistroInfo { id: id?, logo })
}

fn unquote(s: &str) -> String {
    s.trim_matches('"').trim_matches('\'').to_string()
}

fn bundled_icon_for_distro(id: &str) -> Option<&'static str> {
    Some(match id {
        "alpine" => "si-alpinelinux-symbolic",
        "almalinux" => "si-almalinux-symbolic",
        "arch" => "si-archlinux-symbolic",
        "artix" => "si-artixlinux-symbolic",
        "asahi" => "si-asahilinux-symbolic",
        "cachyos" => "cm-cachyos-symbolic",
        "centos" => "si-centos-symbolic",
        "debian" => "si-debian-symbolic",
        "deepin" => "si-deepin-symbolic",
        "elementary" => "si-elementary-symbolic",
        "endeavouros" => "si-endeavouros-symbolic",
        "fedora" => "si-fedora-symbolic",
        "garuda" => "si-garudalinux-symbolic",
        "gentoo" => "si-gentoo-symbolic",
        "kali" => "si-kalilinux-symbolic",
        "kdeneon" => "si-kdeneon-symbolic",
        "kubuntu" => "si-kubuntu-symbolic",
        "linuxmint" => "si-linuxmint-symbolic",
        "lubuntu" => "si-lubuntu-symbolic",
        "manjaro" => "si-manjaro-symbolic",
        "mx" => "si-mxlinux-symbolic",
        "nixos" => "si-nixos-symbolic",
        "nobara" => "cm-nobara-symbolic",
        "opensuse-leap" | "opensuse-tumbleweed" | "opensuse" => "si-opensuse-symbolic",
        "pop" => "si-popos-symbolic",
        "rhel" => "si-redhat-symbolic",
        "rocky" => "si-rockylinux-symbolic",
        "slackware" => "si-slackware-symbolic",
        "solus" => "si-solus-symbolic",
        "steamos" => "si-steam-symbolic",
        "ubuntu" => "si-ubuntu-symbolic",
        "ubuntumate" => "si-ubuntumate-symbolic",
        "void" => "si-voidlinux-symbolic",
        "xubuntu" => "si-xubuntu-symbolic",
        "zorin" => "si-zorin-symbolic",
        _ => return None,
    })
}

/// Resolves the icon to display. Uses override if provided, otherwise auto-detects.
pub(crate) fn build_icon(icon_override: &str) -> String {
    if !icon_override.is_empty() {
        return icon_override.to_string();
    }

    let Some(distro) = detect_distro() else {
        return FALLBACK_ICON.to_string();
    };

    if let Some(icon) = bundled_icon_for_distro(&distro.id)
        && icon_exists(icon)
    {
        return icon.to_string();
    }

    if let Some(logo) = &distro.logo
        && icon_exists(logo)
    {
        return logo.clone();
    }

    let distributor_logo = format!("distributor-logo-{}", distro.id);
    if icon_exists(&distributor_logo) {
        return distributor_logo;
    }

    FALLBACK_ICON.to_string()
}

fn icon_exists(name: &str) -> bool {
    Display::default()
        .map(|display| IconTheme::for_display(&display).has_icon(name))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unquote_removes_double_quotes() {
        assert_eq!(unquote("\"arch\""), "arch");
    }

    #[test]
    fn unquote_removes_single_quotes() {
        assert_eq!(unquote("'arch'"), "arch");
    }

    #[test]
    fn unquote_no_quotes() {
        assert_eq!(unquote("arch"), "arch");
    }

    #[test]
    fn build_icon_uses_override_when_provided() {
        assert_eq!(build_icon("custom-icon"), "custom-icon");
    }

    #[test]
    fn bundled_icon_maps_arch() {
        assert_eq!(
            bundled_icon_for_distro("arch"),
            Some("si-archlinux-symbolic")
        );
    }

    #[test]
    fn bundled_icon_maps_opensuse_variants() {
        assert_eq!(
            bundled_icon_for_distro("opensuse"),
            Some("si-opensuse-symbolic")
        );
        assert_eq!(
            bundled_icon_for_distro("opensuse-leap"),
            Some("si-opensuse-symbolic")
        );
        assert_eq!(
            bundled_icon_for_distro("opensuse-tumbleweed"),
            Some("si-opensuse-symbolic")
        );
    }

    #[test]
    fn bundled_icon_maps_custom_imports() {
        assert_eq!(
            bundled_icon_for_distro("cachyos"),
            Some("cm-cachyos-symbolic")
        );
        assert_eq!(
            bundled_icon_for_distro("nobara"),
            Some("cm-nobara-symbolic")
        );
    }

    #[test]
    fn bundled_icon_returns_none_for_unknown() {
        assert_eq!(bundled_icon_for_distro("unknown-distro"), None);
    }
}
