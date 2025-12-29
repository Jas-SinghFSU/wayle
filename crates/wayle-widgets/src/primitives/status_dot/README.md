# Status Dot Templates

Small circular indicators for inline status display.

## Available Templates

| Template | CSS Classes | Use Case |
|----------|-------------|----------|
| `StatusDot` | `.status-dot` | Default/neutral state |
| `SuccessDot` | `.status-dot .success` | Online, active, complete |
| `WarningDot` | `.status-dot .warning` | Pending, attention needed |
| `ErrorDot` | `.status-dot .error` | Offline, failed, critical |
| `InfoDot` | `.status-dot .info` | Informational, accent highlight |

## Import

```rust
use wayle_widgets::primitives::status_dot::{
    StatusDot, SuccessDot, WarningDot, ErrorDot, InfoDot,
};
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

### Connection Status Pattern

```rust
view! {
    gtk::Box {
        set_spacing: 8,

        if model.is_connected {
            #[template]
            SuccessDot {}
        } else {
            #[template]
            ErrorDot {}
        }

        gtk::Label {
            #[watch]
            set_label: if model.is_connected { "Connected" } else { "Disconnected" },
        },
    }
}
```

## Notes

Status dots are `gtk::Box` widgets with no children or interactive signals.
They're purely visual indicators meant to accompany text labels.
