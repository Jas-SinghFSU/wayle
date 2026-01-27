pub(crate) struct IconContext<'a> {
    pub(crate) count: usize,
    pub(crate) dnd: bool,
    pub(crate) icon_name: &'a str,
    pub(crate) icon_unread: &'a str,
    pub(crate) icon_dnd: &'a str,
}

pub(crate) fn select_icon(ctx: &IconContext<'_>) -> String {
    if ctx.dnd {
        return ctx.icon_dnd.to_string();
    }

    if ctx.count > 0 {
        return ctx.icon_unread.to_string();
    }

    ctx.icon_name.to_string()
}

pub(crate) fn format_label(count: usize, hide_empty: bool) -> String {
    if hide_empty && count == 0 {
        return String::new();
    }

    format!("{count:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ctx(count: usize, dnd: bool) -> IconContext<'static> {
        IconContext {
            count,
            dnd,
            icon_name: "bell",
            icon_unread: "bell-dot",
            icon_dnd: "bell-off",
        }
    }

    #[test]
    fn dnd_returns_dnd_icon() {
        let ctx = make_ctx(5, true);
        assert_eq!(select_icon(&ctx), "bell-off");
    }

    #[test]
    fn dnd_takes_priority_over_count() {
        let ctx = make_ctx(10, true);
        assert_eq!(select_icon(&ctx), "bell-off");
    }

    #[test]
    fn unread_returns_unread_icon() {
        let ctx = make_ctx(3, false);
        assert_eq!(select_icon(&ctx), "bell-dot");
    }

    #[test]
    fn empty_returns_normal_icon() {
        let ctx = make_ctx(0, false);
        assert_eq!(select_icon(&ctx), "bell");
    }

    #[test]
    fn label_shows_count_zero_padded() {
        assert_eq!(format_label(5, false), "05");
        assert_eq!(format_label(0, false), "00");
        assert_eq!(format_label(12, false), "12");
        assert_eq!(format_label(99, false), "99");
        assert_eq!(format_label(100, false), "100");
    }

    #[test]
    fn label_hides_when_empty() {
        assert_eq!(format_label(0, true), "");
        assert_eq!(format_label(5, true), "05");
    }
}
