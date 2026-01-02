//! Radio button group component for single-select options.
#![allow(missing_docs)]

use gtk::prelude::*;
use relm4::prelude::*;

/// Configuration for creating a radio group.
#[derive(Debug, Clone)]
pub struct RadioGroupInit {
    /// Labels for each radio option.
    pub options: Vec<String>,
    /// Initially selected index.
    pub selected: usize,
    /// Layout orientation.
    pub orientation: gtk::Orientation,
}

/// Output messages emitted by the radio group.
#[derive(Debug, Clone)]
pub enum RadioGroupOutput {
    /// Emitted when selection changes to a new option.
    Changed(usize),
}

/// Input messages for controlling the radio group.
#[derive(Debug)]
pub enum RadioGroupMsg {
    /// Internal: a radio button was toggled.
    #[doc(hidden)]
    Toggled(usize),
    /// Set the selected option by index.
    SetSelected(usize),
    /// Enable or disable the entire group.
    SetSensitive(bool),
}

/// Radio button group for mutually exclusive single-select options.
pub struct RadioGroup {
    buttons: Vec<gtk::CheckButton>,
    selected: usize,
    sensitive: bool,
}

#[relm4::component(pub)]
impl SimpleComponent for RadioGroup {
    type Init = RadioGroupInit;
    type Input = RadioGroupMsg;
    type Output = RadioGroupOutput;

    view! {
        #[name = "container"]
        gtk::Box {
            #[watch]
            set_sensitive: model.sensitive,
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut buttons = Vec::with_capacity(init.options.len());
        let mut first_button: Option<gtk::CheckButton> = None;

        for (index, label) in init.options.iter().enumerate() {
            let button = gtk::CheckButton::new();
            button.set_label(Some(label));
            button.set_cursor_from_name(Some("pointer"));

            if let Some(ref first) = first_button {
                button.set_group(Some(first));
            } else {
                first_button = Some(button.clone());
            }

            if index == init.selected {
                button.set_active(true);
            }

            let sender_clone = sender.clone();
            button.connect_toggled(move |btn| {
                if btn.is_active() {
                    sender_clone.input(RadioGroupMsg::Toggled(index));
                }
            });

            buttons.push(button);
        }

        let model = RadioGroup {
            buttons,
            selected: init.selected,
            sensitive: true,
        };

        let widgets = view_output!();

        widgets.container.set_orientation(init.orientation);

        for button in &model.buttons {
            widgets.container.append(button);
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            RadioGroupMsg::Toggled(index) => {
                if index != self.selected {
                    self.selected = index;
                    let _ = sender.output(RadioGroupOutput::Changed(index));
                }
            }
            RadioGroupMsg::SetSelected(index) => {
                if index < self.buttons.len() && index != self.selected {
                    self.buttons[index].set_active(true);
                    self.selected = index;
                }
            }
            RadioGroupMsg::SetSensitive(sensitive) => {
                self.sensitive = sensitive;
            }
        }
    }
}
