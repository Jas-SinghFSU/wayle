mod color_extractor;
mod css;
mod monitors;
mod scss_dev;
mod sysinfo;

use std::env;

use relm4::ComponentSender;

use crate::shell::Shell;

pub(crate) fn init(sender: &ComponentSender<Shell>) {
    css::spawn(sender);
    monitors::spawn(sender);
    color_extractor::spawn();
    sysinfo::spawn();

    if env::var("WAYLE_DEV").is_ok_and(|v| v == "1") {
        scss_dev::spawn(sender);
    }
}
