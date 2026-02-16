use std::{
    any::Any,
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use gtk::prelude::*;
use relm4::{gtk, prelude::*};
use wayle_common::process::{self, ClickAction};
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
    /// opens anchored to the current button.
    pub(crate) fn toggle_for(&self, bar_button: &Controller<BarButton>) {
        let widget = bar_button.widget();
        let widget_ref = widget.upcast_ref::<gtk::Widget>();

        if self.popover.is_visible() {
            if self.popover.parent().as_ref() == Some(widget_ref) {
                self.popover.popdown();
            } else {
                self.reparent_and_show(bar_button);
            }
        } else {
            self.ensure_parent(widget_ref);
            self.freeze_and_show(bar_button);
        }
    }

    fn reparent_and_show(&self, bar_button: &Controller<BarButton>) {
        if let Some(sender) = self.thaw_target.take() {
            sender.emit(BarButtonInput::ThawSize);
        }
        self.popover.unparent();
        self.freeze_and_show(bar_button);
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

    fn freeze_and_show(&self, bar_button: &Controller<BarButton>) {
        let widget = bar_button.widget();
        if self.popover.parent().is_none() {
            self.popover.set_parent(widget);
        }

        self.thaw_target.set(Some(bar_button.sender().clone()));
        bar_button.emit(BarButtonInput::FreezeSize);

        self.apply_position();
        self.lock_parent_size();
        self.popover.popup();
    }

    fn apply_position(&self) {
        let Some(parent) = self.popover.parent() else {
            return;
        };
        self.popover
            .set_position(Self::detect_popover_position(&parent));
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

const REM_PX: f32 = 16.0;

/// Pixel margins applied to dropdown containers.
///
/// Values are rounded to whole pixels so popover content stays visually crisp.
#[derive(Debug, Clone, Copy)]
pub(crate) struct DropdownMargins {
    pub top: i32,
    pub side: i32,
    pub bottom: i32,
}

impl DropdownMargins {
    const TOP_REM: f32 = 0.5;
    const SIDE_REM: f32 = 1.0;
    const BOTTOM_REM: f32 = 1.0;

    pub(crate) fn from_scale(scale: f32) -> Self {
        Self {
            top: Self::round(Self::TOP_REM, scale),
            side: Self::round(Self::SIDE_REM, scale),
            bottom: Self::round(Self::BOTTOM_REM, scale),
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
                dropdown.toggle_for(bar_button);
            }
        }
        ClickAction::Shell(cmd) => process::run_if_set(cmd),
        ClickAction::None => {}
    }
}
