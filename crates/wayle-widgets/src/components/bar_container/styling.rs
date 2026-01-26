//! CSS variable generation for bar container styling.

use relm4::{ComponentSender, gtk};
use wayle_common::watch;
use wayle_config::schemas::styling::ThemeProvider;

use super::component::{BarContainer, BarContainerCmd};
use crate::styling::{InlineStyling, resolve_color};

impl InlineStyling for BarContainer {
    type Sender = ComponentSender<Self>;
    type Cmd = BarContainerCmd;

    fn css_provider(&self) -> &gtk::CssProvider {
        &self.css_provider
    }

    fn spawn_style_watcher(&self, sender: &Self::Sender) {
        let background = self.colors.background.clone();
        let border_color = self.colors.border_color.clone();
        let show_border = self.behavior.show_border.clone();
        let visible = self.behavior.visible.clone();
        let theme_provider = self.theme_provider.clone();
        let border_width = self.border_width.clone();
        let border_location = self.border_location.clone();

        watch!(
            sender,
            [
                background.watch(),
                border_color.watch(),
                show_border.watch(),
                visible.watch(),
                theme_provider.watch(),
                border_width.watch(),
                border_location.watch(),
            ],
            |out| {
                let _ = out.send(BarContainerCmd::ConfigChanged);
            }
        );
    }

    fn build_css(&self) -> String {
        let is_wayle = matches!(self.theme_provider.get(), ThemeProvider::Wayle);

        let bg = resolve_color(&self.colors.background, is_wayle);
        let border_color = resolve_color(&self.colors.border_color, is_wayle);
        let border_width = if self.behavior.show_border.get() {
            self.border_width.get()
        } else {
            0
        };

        format!(
            "* {{ \
             --bar-container-bg: {}; \
             --bar-container-border-color: {}; \
             --bar-container-border-width: {}px; \
             }}",
            bg, border_color, border_width
        )
    }
}
