# Progress Ring

Circular progress indicator with optional center label.

## Sizes

| Variant | Enum         | Dimensions | Stroke |
| ------- | ------------ | ---------- | ------ |
| Small   | `Size::Sm`   | 2rem       | 2px    |
| Medium  | `Size::Md`   | 3rem       | 3px    |
| Large   | `Size::Lg`   | 4rem       | 4px    |
| XL      | `Size::Xl`   | 5rem       | 5px    |
| 2XL     | `Size::Xxl`  | 6rem       | 6px    |
| 3XL     | `Size::Xxxl` | 7rem       | 7px    |

## Colors

| Variant | Enum                    | Use Case             |
| ------- | ----------------------- | -------------------- |
| Default | `ColorVariant::Default` | Accent color         |
| Success | `ColorVariant::Success` | Completion, positive |
| Warning | `ColorVariant::Warning` | Caution states       |
| Error   | `ColorVariant::Error`   | Failed, negative     |

## Import

```rust
use wayle_widgets::primitives::progress_ring::{
    ColorVariant, ProgressRing, ProgressRingInit, ProgressRingMsg, Size,
};
```

## Usage

### Basic

```rust
let ring = ProgressRing::builder()
    .launch(ProgressRingInit {
        fraction: 0.75,
        ..Default::default()
    })
    .detach();
```

### With Size and Color

```rust
let ring = ProgressRing::builder()
    .launch(ProgressRingInit {
        fraction: 0.85,
        size: Size::Lg,
        color: ColorVariant::Success,
    })
    .detach();
```

## Messages

| Message                  | Effect                              |
| ------------------------ | ----------------------------------- |
| `SetFraction(f64)`       | Updates progress (0.0-1.0, clamped) |
| `SetLabel(String)`       | Updates center label text           |
| `SetColor(ColorVariant)` | Updates color variant               |

## Dynamic State

Update all properties at runtime based on changing values:

```rust
fn update(&mut self, msg: Msg, _sender: ComponentSender<Self>) {
    match msg {
        Msg::TemperatureChanged(temp) => {
            let color = if temp > 80.0 {
                ColorVariant::Error
            } else if temp > 60.0 {
                ColorVariant::Warning
            } else {
                ColorVariant::Success
            };

            self.ring.emit(ProgressRingMsg::SetFraction(temp / 100.0));
            self.ring.emit(ProgressRingMsg::SetColor(color));
            self.ring.emit(ProgressRingMsg::SetLabel(format!("{}C", temp)));
        }
    }
}
```

## CSS Classes

- `.progress-ring` - Container element
- `.progress-ring-canvas` - Drawing surface
- `.ring-text` - Center label
