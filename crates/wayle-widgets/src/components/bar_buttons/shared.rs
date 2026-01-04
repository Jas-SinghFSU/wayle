//! Shared utilities for bar button variants.

use std::{borrow::Cow, sync::Arc};

use glib::{Propagation, object::IsA};
use gtk4::{
    EventControllerScroll, EventControllerScrollFlags, GestureClick,
    prelude::{GestureSingleExt, WidgetExt},
};
use relm4::Sender;
use wayle_common::ConfigProperty;
use wayle_config::schemas::styling::ColorValue;

use super::types::BarButtonOutput;

/// Attaches a click gesture that handles left, middle, and right clicks.
///
/// Emits `BarButtonOutput::LeftClick`, `MiddleClick`, or `RightClick` via
/// the provided sender.
pub fn attach_click_gesture(widget: &impl IsA<gtk4::Widget>, sender: Sender<BarButtonOutput>) {
    let click = GestureClick::new();
    click.set_button(0);

    click.connect_pressed(move |gesture, _n_press, _x, _y| {
        let event = match gesture.current_button() {
            1 => BarButtonOutput::LeftClick,
            2 => BarButtonOutput::MiddleClick,
            3 => BarButtonOutput::RightClick,
            _ => return,
        };
        let _ = sender.send(event);
    });

    widget.add_controller(click);
}

/// Attaches a scroll controller with configurable sensitivity.
///
/// Emits `BarButtonOutput::ScrollUp` or `ScrollDown` via the provided sender.
/// Higher sensitivity values make scrolling trigger more easily.
pub fn attach_scroll_controller(
    widget: &impl IsA<gtk4::Widget>,
    sender: Sender<BarButtonOutput>,
    sensitivity: f64,
) {
    let scroll = EventControllerScroll::new(EventControllerScrollFlags::VERTICAL);
    let threshold = 0.5 / sensitivity.max(0.1);

    scroll.connect_scroll(move |_controller, _dx, dy| {
        if dy.abs() < threshold {
            return Propagation::Proceed;
        }
        let event = if dy < 0.0 {
            BarButtonOutput::ScrollUp
        } else {
            BarButtonOutput::ScrollDown
        };
        let _ = sender.send(event);
        Propagation::Stop
    });

    widget.add_controller(scroll);
}

/// Sets up both click and scroll controllers on a widget.
pub fn setup_event_controllers(
    widget: &impl IsA<gtk4::Widget>,
    sender: Sender<BarButtonOutput>,
    scroll_sensitivity: f64,
) {
    attach_click_gesture(widget, sender.clone());
    attach_scroll_controller(widget, sender, scroll_sensitivity);
}

/// Resolves a color value to CSS, respecting the theme provider setting.
///
/// When Wayle theming is active, returns the configured color value.
/// Otherwise, falls back to the property's default value for consistent
/// appearance with external theme providers.
pub fn resolve_color(
    color_prop: &Arc<ConfigProperty<ColorValue>>,
    is_wayle_themed: bool,
) -> Cow<'static, str> {
    if is_wayle_themed {
        color_prop.get().to_css()
    } else {
        color_prop.default().to_css()
    }
}
