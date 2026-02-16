use relm4::ComponentSender;
use wayle_common::{watch, watchers::changes_stream};

use crate::shell::{Shell, ShellCmd, ShellServices};

pub(crate) fn spawn(sender: &ComponentSender<Shell>, services: &ShellServices) {
    let config = services.config.config().clone();

    watch!(sender, [changes_stream(&config.bar.location)], |out| {
        let _ = out.send(ShellCmd::LocationChanged);
    });
}
