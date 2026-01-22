//! CSS variable generation for bar button styling.

use relm4::{ComponentSender, gtk};
use tokio::sync::mpsc;
use wayle_common::SubscribeChanges;
use wayle_config::schemas::styling::ThemeProvider;

use super::component::{BarButton, BarButtonCmd};
use crate::styling::{InlineStyling, resolve_color};

impl InlineStyling for BarButton {
    type Sender = ComponentSender<Self>;
    type Cmd = BarButtonCmd;

    fn css_provider(&self) -> &gtk::CssProvider {
        &self.css_provider
    }

    fn spawn_style_watcher(&self, sender: &Self::Sender) {
        let (tx, mut rx) = mpsc::unbounded_channel();

        self.behavior.show_icon.subscribe_changes(tx.clone());
        self.behavior.show_label.subscribe_changes(tx.clone());
        self.behavior.show_border.subscribe_changes(tx.clone());
        self.behavior.visible.subscribe_changes(tx.clone());
        self.behavior.label_max_chars.subscribe_changes(tx.clone());
        self.colors.icon_color.subscribe_changes(tx.clone());
        self.colors.label_color.subscribe_changes(tx.clone());
        self.colors.icon_background.subscribe_changes(tx.clone());
        self.colors.button_background.subscribe_changes(tx.clone());
        self.colors.border_color.subscribe_changes(tx.clone());
        self.settings.border_location.subscribe_changes(tx.clone());
        self.settings.border_width.subscribe_changes(tx.clone());
        self.settings.theme_provider.subscribe_changes(tx.clone());
        self.settings.is_vertical.subscribe_changes(tx);

        sender.command(move |out, shutdown| async move {
            let shutdown_fut = shutdown.wait();
            tokio::pin!(shutdown_fut);

            loop {
                tokio::select! {
                    () = &mut shutdown_fut => break,
                    Some(()) = rx.recv() => {
                        let _ = out.send(BarButtonCmd::ConfigChanged);
                    }
                }
            }
        });
    }

    fn build_css(&self) -> String {
        let is_wayle = matches!(self.settings.theme_provider.get(), ThemeProvider::Wayle);

        let icon_color = self.resolve_icon_color(is_wayle);
        let label_color = resolve_color(&self.colors.label_color, is_wayle);
        let icon_bg = resolve_color(&self.colors.icon_background, is_wayle);
        let button_bg = resolve_color(&self.colors.button_background, is_wayle);
        let border_color = resolve_color(&self.colors.border_color, is_wayle);
        let border_width = self.settings.border_width.get();

        format!(
            "* {{ \
             --bar-btn-icon-color: {}; \
             --bar-btn-label-color: {}; \
             --bar-btn-icon-bg: {}; \
             --bar-btn-bg: {}; \
             --bar-btn-border-color: {}; \
             --bar-btn-border-width: {}px; \
             }}",
            icon_color, label_color, icon_bg, button_bg, border_color, border_width
        )
    }
}
