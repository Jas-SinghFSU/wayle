//! CSS variable generation for bar styling.

use relm4::gtk;
use wayle_common::services;
use wayle_config::{
    ConfigService,
    schemas::{bar::BorderLocation, styling::ThemeProvider},
};
use wayle_widgets::styling::{InlineStyling, resolve_color};

use super::Bar;

impl InlineStyling for Bar {
    fn css_provider(&self) -> &gtk::CssProvider {
        &self.css_provider
    }

    fn build_css(&self) -> String {
        let config_service = services::get::<ConfigService>();
        let config = config_service.config();
        let bar = &config.bar;
        let styling = &config.styling;
        let is_wayle = matches!(styling.theme_provider.get(), ThemeProvider::Wayle);

        let bg = resolve_color(&bar.bg, is_wayle);
        let bg_opacity = bar.background_opacity.get().value();
        let border_color = resolve_color(&bar.border_color, is_wayle);
        let border_width = bar.border_width.get();
        let border_location = bar.border_location.get();

        let (border_top, border_bottom, border_left, border_right) = match border_location {
            BorderLocation::None => (0, 0, 0, 0),
            BorderLocation::Top => (border_width, 0, 0, 0),
            BorderLocation::Bottom => (0, border_width, 0, 0),
            BorderLocation::Left => (0, 0, border_width, 0),
            BorderLocation::Right => (0, 0, 0, border_width),
            BorderLocation::All => (border_width, border_width, border_width, border_width),
        };

        let scale = bar.scale.get().value();
        let inset_edge = bar.inset_edge.get().value();
        let inset_ends = bar.inset_ends.get().value();
        let padding = bar.padding.get().value();
        let padding_ends = bar.padding_ends.get().value();
        let module_gap = bar.module_gap.get().value();
        let group_module_gap = bar.button_group_module_gap.get().value();

        let location = bar.location.get();
        let shadow_preset = bar.shadow.get();
        let shadow = shadow_preset.css_shadow(location);
        let shadow_margin = shadow_preset.opposite_margin();

        format!(
            ".bar {{ \
            --bar-scale: {scale}; \
            --bar-bg: {bg}; \
            --bar-opacity: {bg_opacity}%; \
            --bar-border-color: {border_color}; \
            --bar-border-top: {border_top}; \
            --bar-border-bottom: {border_bottom}; \
            --bar-border-left: {border_left}; \
            --bar-border-right: {border_right}; \
            --bar-inset-edge: {inset_edge}; \
            --bar-inset-ends: {inset_ends}; \
            --bar-padding: {padding}; \
            --bar-padding-ends: {padding_ends}; \
            --bar-module-gap: {module_gap}; \
            --bar-group-module-gap: {group_module_gap}; \
            --bar-shadow: {shadow}; \
            --bar-shadow-margin: {shadow_margin}; \
            }}"
        )
    }
}
