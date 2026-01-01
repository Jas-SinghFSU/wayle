# Slider

Range sliders for value selection.

## Available

| Type      | Name            | Use Case                              |
| --------- | --------------- | ------------------------------------- |
| Template  | `Slider`        | Volume, brightness, continuous ranges |
| Component | `SteppedSlider` | Discrete values with snapping         |

## Import

```rust
use wayle_widgets::primitives::slider::Slider;
use wayle_widgets::primitives::slider::{
    SteppedSlider, SteppedSliderInit, SteppedSliderMsg, SteppedSliderOutput, EmitMode,
};
```

## Slider Template

### Basic

```rust
view! {
    #[template]
    Slider {
        set_range: (0.0, 100.0),
        set_value: 50.0,
    }
}
```

### With Signal

```rust
view! {
    #[template]
    Slider {
        set_range: (0.0, 100.0),
        #[watch]
        set_value: model.volume,
        connect_value_changed[sender] => move |scale| {
            sender.input(Msg::VolumeChanged(scale.value()));
        },
    }
}
```

## SteppedSlider Component

### Setup

```rust
struct App {
    stepped_slider: Controller<SteppedSlider>,
}

fn init(...) -> ComponentParts<Self> {
    let stepped_slider = SteppedSlider::builder()
        .launch(SteppedSliderInit {
            range: (0.0, 100.0),
            value: 50.0,
            steps: vec![0.0, 25.0, 50.0, 75.0, 100.0],
            show_labels: true,
            emit_mode: EmitMode::Continuous,
        })
        .forward(sender.input_sender(), |output| match output {
            SteppedSliderOutput::Changed(value) => Msg::StepChanged(value),
        });

    // ...
}
```

### SteppedSliderInit Fields

| Field         | Type         | Default        | Purpose              |
| ------------- | ------------ | -------------- | -------------------- |
| `range`       | `(f64, f64)` | `(0.0, 100.0)` | Value range          |
| `value`       | `f64`        | `50.0`         | Initial value        |
| `steps`       | `Vec<f64>`   | 5 steps        | Snap points          |
| `show_labels` | `bool`       | `false`        | Show step labels     |
| `emit_mode`   | `EmitMode`   | `Continuous`   | When to emit changes |

### EmitMode

| Mode         | Behavior             |
| ------------ | -------------------- |
| `Continuous` | Emit on every drag   |
| `OnRelease`  | Emit only on release |

### Messages

```rust
// Set value externally
self.stepped_slider.emit(SteppedSliderMsg::SetValue(75.0));

// Enable/disable
self.stepped_slider.emit(SteppedSliderMsg::SetSensitive(false));
```
