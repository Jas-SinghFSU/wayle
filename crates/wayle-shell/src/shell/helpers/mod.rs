mod bootstrap;
mod monitors;

pub(crate) use bootstrap::{create_bars, init_css_provider, init_icons, register_app_actions};
pub(crate) use monitors::get_current_monitors;
