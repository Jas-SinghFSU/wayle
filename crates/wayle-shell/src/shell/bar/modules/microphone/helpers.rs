pub(crate) struct IconContext<'a> {
    pub(crate) muted: bool,
    pub(crate) icon_active: &'a str,
    pub(crate) icon_muted: &'a str,
}

pub(crate) fn format_label(percentage: u16) -> String {
    format!("{percentage}%")
}

pub(crate) fn select_icon(ctx: &IconContext<'_>) -> String {
    if ctx.muted {
        ctx.icon_muted.to_string()
    } else {
        ctx.icon_active.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn muted_returns_muted_icon() {
        let result = select_icon(&IconContext {
            muted: true,
            icon_active: "mic",
            icon_muted: "mic-off",
        });
        assert_eq!(result, "mic-off");
    }

    #[test]
    fn unmuted_returns_active_icon() {
        let result = select_icon(&IconContext {
            muted: false,
            icon_active: "mic",
            icon_muted: "mic-off",
        });
        assert_eq!(result, "mic");
    }
}
