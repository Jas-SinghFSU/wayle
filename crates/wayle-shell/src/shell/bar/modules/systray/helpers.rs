use glob::Pattern;
use wayle_config::schemas::modules::{SystrayConfig, TrayItemOverride};
use wayle_systray::core::item::TrayItem;

pub(crate) fn is_blacklisted(item: &TrayItem, config: &SystrayConfig) -> bool {
    let blacklist = config.blacklist.get();
    let id = item.id.get();
    let title = item.title.get();

    blacklist.iter().any(|pattern| {
        let Ok(glob) = Pattern::new(pattern) else {
            return false;
        };
        glob.matches(&id) || glob.matches(&title)
    })
}

pub(crate) fn find_override<'a>(
    item: &TrayItem,
    overrides: &'a [TrayItemOverride],
) -> Option<&'a TrayItemOverride> {
    let id = item.id.get();
    let title = item.title.get();

    overrides.iter().find(|o| {
        let Ok(glob) = Pattern::new(&o.name) else {
            return false;
        };
        glob.matches(&id) || glob.matches(&title)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_matches_exact() {
        let pattern = Pattern::new("discord").unwrap();
        assert!(pattern.matches("discord"));
        assert!(!pattern.matches("Discord"));
    }

    #[test]
    fn pattern_matches_wildcard() {
        let pattern = Pattern::new("*discord*").unwrap();
        assert!(pattern.matches("discord"));
        assert!(pattern.matches("com.discord.Discord"));
        assert!(pattern.matches("discord-canary"));
    }

    #[test]
    fn pattern_case_sensitive() {
        let pattern = Pattern::new("*Discord*").unwrap();
        assert!(pattern.matches("Discord"));
        assert!(!pattern.matches("discord"));
    }
}
