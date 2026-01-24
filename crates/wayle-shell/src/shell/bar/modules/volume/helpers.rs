pub(crate) struct IconContext<'a> {
    pub(crate) percentage: u16,
    pub(crate) muted: bool,
    pub(crate) level_icons: &'a [String],
    pub(crate) muted_icon: &'a str,
}

pub(crate) fn select_icon(ctx: &IconContext<'_>) -> String {
    if ctx.muted {
        return ctx.muted_icon.to_string();
    }

    if ctx.level_icons.is_empty() {
        return ctx.muted_icon.to_string();
    }

    let index = if ctx.percentage == 0 {
        0
    } else {
        let step = 100.0 / ctx.level_icons.len() as f64;
        let idx = ((ctx.percentage as f64 - 1.0) / step).floor() as usize;
        idx.min(ctx.level_icons.len() - 1)
    };

    ctx.level_icons
        .get(index)
        .cloned()
        .unwrap_or_else(|| ctx.muted_icon.to_string())
}

pub(crate) fn format_label(percentage: u16) -> String {
    format!("{percentage}%")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_icons() -> Vec<String> {
        vec![
            String::from("vol-1"),
            String::from("vol-2"),
            String::from("vol-3"),
        ]
    }

    #[test]
    fn muted_returns_muted_icon() {
        let icons = make_icons();
        let result = select_icon(&IconContext {
            percentage: 50,
            muted: true,
            level_icons: &icons,
            muted_icon: "muted",
        });
        assert_eq!(result, "muted");
    }

    #[test]
    fn zero_percent_returns_first_icon() {
        let icons = make_icons();
        let result = select_icon(&IconContext {
            percentage: 0,
            muted: false,
            level_icons: &icons,
            muted_icon: "muted",
        });
        assert_eq!(result, "vol-1");
    }

    #[test]
    fn low_volume_returns_first_icon() {
        let icons = make_icons();
        let result = select_icon(&IconContext {
            percentage: 15,
            muted: false,
            level_icons: &icons,
            muted_icon: "muted",
        });
        assert_eq!(result, "vol-1");
    }

    #[test]
    fn mid_volume_returns_second_icon() {
        let icons = make_icons();
        let result = select_icon(&IconContext {
            percentage: 50,
            muted: false,
            level_icons: &icons,
            muted_icon: "muted",
        });
        assert_eq!(result, "vol-2");
    }

    #[test]
    fn high_volume_returns_last_icon() {
        let icons = make_icons();
        let result = select_icon(&IconContext {
            percentage: 100,
            muted: false,
            level_icons: &icons,
            muted_icon: "muted",
        });
        assert_eq!(result, "vol-3");
    }

    #[test]
    fn empty_icons_returns_muted() {
        let result = select_icon(&IconContext {
            percentage: 50,
            muted: false,
            level_icons: &[],
            muted_icon: "muted",
        });
        assert_eq!(result, "muted");
    }

    #[test]
    fn boosted_volume_returns_last_icon() {
        let icons = make_icons();
        let result = select_icon(&IconContext {
            percentage: 150,
            muted: false,
            level_icons: &icons,
            muted_icon: "muted",
        });
        assert_eq!(result, "vol-3");
    }
}
