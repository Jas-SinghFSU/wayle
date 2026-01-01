# Status Dot Templates

Small circular indicators for inline status display.

## Available

| Template     | CSS Classes            | Use Case                        |
| ------------ | ---------------------- | ------------------------------- |
| `StatusDot`  | `.status-dot`          | Default/neutral state           |
| `SuccessDot` | `.status-dot .success` | Online, active, complete        |
| `WarningDot` | `.status-dot .warning` | Pending, attention needed       |
| `ErrorDot`   | `.status-dot .error`   | Offline, failed, critical       |
| `InfoDot`    | `.status-dot .info`    | Informational, accent highlight |

## Import

```rust
use wayle_widgets::primitives::status_dot::{StatusDot, SuccessDot, WarningDot, ErrorDot, InfoDot};
```

## Usage

### Standalone

```rust
view! {
    #[template]
    SuccessDot {}
}
```

### With Label

```rust
view! {
    gtk::Box {
        set_spacing: 8,

        #[template]
        SuccessDot {},

        gtk::Label {
            set_label: "Online",
        },
    }
}
```
