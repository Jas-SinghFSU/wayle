use std::{
    any::Any,
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use gtk::prelude::*;
use relm4::{gtk, prelude::*};
use wayle_common::process::{self, ClickAction};
use wayle_config::schemas::bar::Location;
use wayle_widgets::prelude::{BarButton, BarButtonInput};

use crate::shell::services::ShellServices;

/// Shared dropdown instance for a dropdown name.
///
/// Reuse keeps dropdown state consistent across modules that reference the same
/// dropdown and avoids rebuilding the same component repeatedly.
pub(crate) struct DropdownInstance {
    popover: gtk::Popover,
    _controller: Box<dyn Any>,
    thaw_target: Rc<Cell<Option<relm4::Sender<BarButtonInput>>>>,
}

impl DropdownInstance {
    pub(crate) fn new(popover: gtk::Popover, controller: Box<dyn Any>) -> Self {
        let thaw_target: Rc<Cell<Option<relm4::Sender<BarButtonInput>>>> = Rc::default();

        let thaw = thaw_target.clone();
        popover.connect_closed(move |popover| {
            if let Some(sender) = thaw.take() {
                sender.emit(BarButtonInput::ThawSize);
            }
            if let Some(parent) = popover.parent() {
                parent.set_size_request(-1, -1);
            }
        });

        Self {
            popover,
            _controller: controller,
            thaw_target,
        }
    }

    /// Toggles popover visibility for the given bar button.
    ///
    /// If the popover is already open for this button, it closes; otherwise it
    /// opens anchored to the current button. Margins are applied from the
    /// registry so individual dropdowns never handle positioning.
    fn toggle_for(&self, bar_button: &Controller<BarButton>, style: DropdownStyle) {
        let widget = bar_button.widget();
        let widget_ref = widget.upcast_ref::<gtk::Widget>();

        if self.popover.is_visible() {
            if self.popover.parent().as_ref() == Some(widget_ref) {
                self.popover.popdown();
            } else {
                self.reparent_and_show(bar_button, style);
            }
        } else {
            self.ensure_parent(widget_ref);
            self.freeze_and_show(bar_button, style);
        }
    }

    fn reparent_and_show(&self, bar_button: &Controller<BarButton>, style: DropdownStyle) {
        if let Some(sender) = self.thaw_target.take() {
            sender.emit(BarButtonInput::ThawSize);
        }
        self.popover.unparent();
        self.freeze_and_show(bar_button, style);
    }

    fn ensure_parent(&self, target: &gtk::Widget) {
        if self.popover.parent().as_ref() == Some(target) {
            return;
        }
        if self.popover.parent().is_some() {
            self.popover.unparent();
        }
        self.popover.set_parent(target);
    }

    fn freeze_and_show(&self, bar_button: &Controller<BarButton>, style: DropdownStyle) {
        let widget = bar_button.widget();
        if self.popover.parent().is_none() {
            self.popover.set_parent(widget);
        }

        self.thaw_target.set(Some(bar_button.sender().clone()));
        bar_button.emit(BarButtonInput::FreezeSize);

        self.apply_position();
        self.apply_margins(style.margins);
        self.apply_style(&style);
        self.lock_parent_size();
        self.popover.popup();
    }

    fn apply_style(&self, style: &DropdownStyle) {
        self.popover.set_opacity(style.opacity);
        if style.shadow_enabled {
            self.popover.add_css_class("shadow");
        } else {
            self.popover.remove_css_class("shadow");
        }
    }

    fn apply_position(&self) {
        let Some(parent) = self.popover.parent() else {
            return;
        };
        let position = Self::detect_popover_position(&parent);
        self.popover.set_position(position);

        for class in &[
            "position-top",
            "position-bottom",
            "position-left",
            "position-right",
        ] {
            self.popover.remove_css_class(class);
        }
        let class = match position {
            gtk::PositionType::Top => "position-top",
            gtk::PositionType::Bottom => "position-bottom",
            gtk::PositionType::Left => "position-left",
            gtk::PositionType::Right => "position-right",
            _ => "position-bottom",
        };
        self.popover.add_css_class(class);
    }

    fn apply_margins(&self, margins: DropdownMargins) {
        let Some(child) = self.popover.child() else {
            return;
        };
        child.set_margin_top(margins.top);
        child.set_margin_bottom(margins.bottom);
        child.set_margin_start(margins.start);
        child.set_margin_end(margins.end);
    }

    fn lock_parent_size(&self) {
        let Some(parent) = self.popover.parent() else {
            return;
        };
        parent.set_size_request(parent.width(), parent.height());
    }

    fn detect_popover_position(widget: &gtk::Widget) -> gtk::PositionType {
        let Some(window) = widget.root().and_then(|r| r.downcast::<gtk::Window>().ok()) else {
            return gtk::PositionType::Bottom;
        };

        if window.has_css_class("bottom") {
            gtk::PositionType::Top
        } else if window.has_css_class("left") {
            gtk::PositionType::Right
        } else if window.has_css_class("right") {
            gtk::PositionType::Left
        } else {
            gtk::PositionType::Bottom
        }
    }
}

impl Drop for DropdownInstance {
    fn drop(&mut self) {
        self.popover.unparent();
    }
}

struct DropdownStyle {
    margins: DropdownMargins,
    opacity: f64,
    shadow_enabled: bool,
}

const REM_PX: f32 = 16.0;

/// Pixel margins applied to dropdown containers.
///
/// Values are rounded to whole pixels so popover content stays visually crisp.
/// The bar-facing edge gets a smaller gap; the opposite edge and sides get
/// standard content padding.
#[derive(Debug, Clone, Copy)]
struct DropdownMargins {
    top: i32,
    bottom: i32,
    start: i32,
    end: i32,
}

impl DropdownMargins {
    const GAP_REM: f32 = 0.275;
    const CONTENT_REM: f32 = 1.0;

    fn new(scale: f32, location: Location) -> Self {
        let gap = Self::round(Self::GAP_REM, scale);
        let content = Self::round(Self::CONTENT_REM, scale);

        match location {
            Location::Top => Self {
                top: gap,
                bottom: content,
                start: content,
                end: content,
            },
            Location::Bottom => Self {
                top: content,
                bottom: gap,
                start: content,
                end: content,
            },
            Location::Left => Self {
                top: content,
                bottom: content,
                start: gap,
                end: content,
            },
            Location::Right => Self {
                top: content,
                bottom: content,
                start: content,
                end: gap,
            },
        }
    }

    fn round(rem: f32, scale: f32) -> i32 {
        (rem * REM_PX * scale).round() as i32
    }
}

/// Factory trait for creating dropdown component instances.
pub(crate) trait DropdownFactory {
    /// Creates a dropdown component, returning `None` if required services are unavailable.
    fn create(services: &ShellServices) -> Option<DropdownInstance>;
}

/// Cache of dropdown instances keyed by dropdown name.
///
/// Dropdowns are created lazily on first use and reused afterward so repeated
/// interactions resolve to the same logical dropdown instance.
pub(crate) struct DropdownRegistry {
    services: ShellServices,
    cache: RefCell<HashMap<String, Rc<DropdownInstance>>>,
}

impl DropdownRegistry {
    pub(crate) fn new(services: &ShellServices) -> Self {
        Self {
            services: services.clone(),
            cache: RefCell::default(),
        }
    }

    fn get_or_create(&self, name: &str) -> Option<Rc<DropdownInstance>> {
        let mut cache = self.cache.borrow_mut();
        if let Some(instance) = cache.get(name) {
            return Some(instance.clone());
        }

        let instance = Rc::new(super::create(name, &self.services)?);
        cache.insert(name.to_owned(), instance.clone());
        Some(instance)
    }
}

/// Dispatches a click action: toggles dropdown, runs shell command, or no-ops.
pub(crate) fn dispatch_click(
    action: &ClickAction,
    registry: &DropdownRegistry,
    bar_button: &Controller<BarButton>,
) {
    match action {
        ClickAction::Dropdown(name) => {
            if let Some(dropdown) = registry.get_or_create(name) {
                let config = registry.services.config.config();
                let bar = &config.bar;
                let scale = bar.scale.get().value();
                let style = DropdownStyle {
                    margins: DropdownMargins::new(scale, bar.location.get()),
                    opacity: f64::from(bar.dropdown_opacity.get().value()) / 100.0,
                    shadow_enabled: bar.dropdown_shadow.get(),
                };
                dropdown.toggle_for(bar_button, style);
            }
        }
        ClickAction::Shell(cmd) => process::run_if_set(cmd),
        ClickAction::None => {}
    }
}
