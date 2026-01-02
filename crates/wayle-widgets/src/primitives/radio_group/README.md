# RadioGroup

Mutually exclusive single-select options using grouped radio buttons.

## Import

```rust
use wayle_widgets::primitives::radio_group::{
    RadioGroup, RadioGroupInit, RadioGroupMsg, RadioGroupOutput,
};
```

## Usage

### Basic

```rust
use relm4::Controller;

struct MyComponent {
    radio_group: Controller<RadioGroup>,
    selected_theme: usize,
}

fn init(...) {
    let radio_group = RadioGroup::builder()
        .launch(RadioGroupInit {
            options: vec![
                "Light".to_string(),
                "Dark".to_string(),
                "System".to_string(),
            ],
            selected: 0,
            orientation: gtk::Orientation::Vertical,
        })
        .forward(sender.input_sender(), |output| match output {
            RadioGroupOutput::Changed(index) => Msg::ThemeChanged(index),
        });
}
```

### Horizontal Layout

```rust
RadioGroupInit {
    options: vec!["Small".to_string(), "Medium".to_string(), "Large".to_string()],
    selected: 1,
    orientation: gtk::Orientation::Horizontal,
}
```

### Programmatic Control

```rust
fn update(&mut self, msg: Msg, sender: ComponentSender<Self>) {
    match msg {
        Msg::SetTheme(index) => {
            self.radio_group.emit(RadioGroupMsg::SetSelected(index));
        }
        Msg::DisableOptions => {
            self.radio_group.emit(RadioGroupMsg::SetSensitive(false));
        }
    }
}
```

## Init

| Field         | Type               | Description                   |
| ------------- | ------------------ | ----------------------------- |
| `options`     | `Vec<String>`      | Labels for each radio option  |
| `selected`    | `usize`            | Initially selected index      |
| `orientation` | `gtk::Orientation` | Vertical or Horizontal layout |

## Messages

### Input (`RadioGroupMsg`)

| Variant              | Description                                |
| -------------------- | ------------------------------------------ |
| `SetSelected(usize)` | Programmatically select an option by index |
| `SetSensitive(bool)` | Enable or disable the entire group         |

### Output (`RadioGroupOutput`)

| Variant          | Description                                  |
| ---------------- | -------------------------------------------- |
| `Changed(usize)` | Emitted when user selects a different option |

## Embedding in View

```rust
view! {
    gtk::Box {
        set_orientation: gtk::Orientation::Vertical,
        model.radio_group.widget(),
    }
}
```
