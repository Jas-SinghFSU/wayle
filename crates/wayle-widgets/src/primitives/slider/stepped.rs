//! Stepped slider component with discrete value snapping.
#![allow(missing_docs)]

use gtk::{glib, prelude::*};
use relm4::prelude::*;

/// Controls when the slider emits value changes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum EmitMode {
    /// Emit on every value change during drag.
    #[default]
    Continuous,
    /// Emit only when the user releases the slider.
    OnRelease,
}

/// Configuration for creating a stepped slider.
#[derive(Debug, Clone)]
pub struct SteppedSliderInit {
    /// Slider range as (min, max).
    pub range: (f64, f64),
    /// Initial value (will be snapped to nearest step).
    pub value: f64,
    /// Discrete step values. Must contain at least 2 values.
    pub steps: Vec<f64>,
    /// Whether to display labels under each step mark.
    pub show_labels: bool,
    /// When to emit value changes.
    pub emit_mode: EmitMode,
}

impl Default for SteppedSliderInit {
    fn default() -> Self {
        Self {
            range: (0.0, 100.0),
            value: 50.0,
            steps: vec![0.0, 25.0, 50.0, 75.0, 100.0],
            show_labels: false,
            emit_mode: EmitMode::default(),
        }
    }
}

/// Output messages emitted by the stepped slider.
#[derive(Debug, Clone)]
pub enum SteppedSliderOutput {
    /// Emitted when the value changes to a new step.
    Changed(f64),
}

/// Input messages for controlling the stepped slider.
#[derive(Debug)]
pub enum SteppedSliderMsg {
    /// Set the slider value (will be snapped to nearest step).
    SetValue(f64),
    /// Enable or disable the slider.
    SetSensitive(bool),
}

/// Stepped slider component with discrete value snapping and marks.
pub struct SteppedSlider {
    value: f64,
    steps: Vec<f64>,
    sensitive: bool,
}

fn snap_to_nearest(value: f64, steps: &[f64]) -> f64 {
    steps
        .iter()
        .min_by(|a, b| {
            let dist_a = (value - *a).abs();
            let dist_b = (value - *b).abs();
            dist_a
                .partial_cmp(&dist_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .copied()
        .unwrap_or(value)
}

#[relm4::component(pub)]
impl SimpleComponent for SteppedSlider {
    type Init = SteppedSliderInit;
    type Input = SteppedSliderMsg;
    type Output = SteppedSliderOutput;

    view! {
        #[name = "scale"]
        gtk::Scale {
            set_draw_value: false,
            set_cursor_from_name: Some("pointer"),
            set_has_origin: true,
            #[watch]
            set_value: model.value,
            #[watch]
            set_sensitive: model.sensitive,
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = SteppedSlider {
            value: snap_to_nearest(init.value, &init.steps),
            steps: init.steps.clone(),
            sensitive: true,
        };

        let widgets = view_output!();

        widgets.scale.set_range(init.range.0, init.range.1);
        widgets.scale.set_value(model.value);

        for step in &init.steps {
            let label = init.show_labels.then(|| step.to_string());
            widgets
                .scale
                .add_mark(*step, gtk::PositionType::Bottom, label.as_deref());
        }

        let steps = init.steps.clone();
        let emit_mode = init.emit_mode;
        let output_sender = sender.output_sender().clone();

        widgets
            .scale
            .connect_change_value(move |scale, _, raw_value| {
                let snapped = snap_to_nearest(raw_value, &steps);
                scale.set_value(snapped);
                if emit_mode == EmitMode::Continuous {
                    let _ = output_sender.send(SteppedSliderOutput::Changed(snapped));
                }
                glib::Propagation::Stop
            });

        if init.emit_mode == EmitMode::OnRelease {
            let steps = init.steps;
            let output_sender = sender.output_sender().clone();
            let scale = widgets.scale.clone();

            let legacy = gtk::EventControllerLegacy::new();
            legacy.connect_event(move |_, event| {
                if event.event_type() == gtk::gdk::EventType::ButtonRelease {
                    let current = scale.value();
                    let snapped = snap_to_nearest(current, &steps);
                    let _ = output_sender.send(SteppedSliderOutput::Changed(snapped));
                }
                glib::Propagation::Proceed
            });
            widgets.scale.add_controller(legacy);
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            SteppedSliderMsg::SetValue(value) => {
                self.value = snap_to_nearest(value, &self.steps);
            }
            SteppedSliderMsg::SetSensitive(sensitive) => {
                self.sensitive = sensitive;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snaps_to_nearest_step() {
        let steps = vec![0.0, 25.0, 50.0, 75.0, 100.0];

        assert_eq!(snap_to_nearest(0.0, &steps), 0.0);
        assert_eq!(snap_to_nearest(12.0, &steps), 0.0);
        assert_eq!(snap_to_nearest(13.0, &steps), 25.0);
        assert_eq!(snap_to_nearest(37.0, &steps), 25.0);
        assert_eq!(snap_to_nearest(38.0, &steps), 50.0);
        assert_eq!(snap_to_nearest(100.0, &steps), 100.0);
    }

    #[test]
    fn snaps_equidistant_to_lower() {
        let steps = vec![0.0, 50.0, 100.0];

        assert_eq!(snap_to_nearest(25.0, &steps), 0.0);
        assert_eq!(snap_to_nearest(75.0, &steps), 50.0);
    }

    #[test]
    fn handles_single_step() {
        let steps = vec![50.0];

        assert_eq!(snap_to_nearest(0.0, &steps), 50.0);
        assert_eq!(snap_to_nearest(100.0, &steps), 50.0);
    }

    #[test]
    fn handles_empty_steps_returns_original() {
        let steps: Vec<f64> = vec![];

        assert_eq!(snap_to_nearest(42.0, &steps), 42.0);
    }

    #[test]
    fn handles_values_outside_range() {
        let steps = vec![25.0, 50.0, 75.0];

        assert_eq!(snap_to_nearest(-100.0, &steps), 25.0);
        assert_eq!(snap_to_nearest(200.0, &steps), 75.0);
    }

    #[test]
    fn handles_unordered_steps() {
        let steps = vec![75.0, 25.0, 50.0, 0.0, 100.0];

        assert_eq!(snap_to_nearest(30.0, &steps), 25.0);
        assert_eq!(snap_to_nearest(60.0, &steps), 50.0);
    }

    #[test]
    fn default_init_has_expected_values() {
        let init = SteppedSliderInit::default();

        assert_eq!(init.range, (0.0, 100.0));
        assert_eq!(init.value, 50.0);
        assert_eq!(init.steps, vec![0.0, 25.0, 50.0, 75.0, 100.0]);
        assert!(!init.show_labels);
        assert_eq!(init.emit_mode, EmitMode::Continuous);
    }
}
