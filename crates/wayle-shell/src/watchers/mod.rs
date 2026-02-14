mod color_extractor;
mod css;
mod monitors;
mod scss_dev;
mod sysinfo;
mod wallpaper;
mod weather;

use std::env;

use relm4::ComponentSender;

use crate::shell::{Shell, ShellServices};

pub(crate) fn init(sender: &ComponentSender<Shell>, services: &ShellServices) {
    css::spawn(sender, services);
    monitors::spawn(sender);
    color_extractor::spawn(services);
    sysinfo::spawn(services);
    wallpaper::spawn(services);
    weather::spawn(services);

    if env::var("WAYLE_DEV").is_ok_and(|value| value == "1") {
        scss_dev::spawn(sender, services);
    }
}
