# Alert Templates

Status messages and notifications.

## Available

| Template       | CSS Classes       | Use Case                |
| -------------- | ----------------- | ----------------------- |
| `Alert`        | `.alert`          | Neutral status messages |
| `SuccessAlert` | `.alert .success` | Positive confirmations  |
| `WarningAlert` | `.alert .warning` | Caution messages        |
| `ErrorAlert`   | `.alert .error`   | Failure messages        |
| `InfoAlert`    | `.alert .info`    | Informational messages  |

## Import

```rust
use wayle_widgets::primitives::alert::{Alert, SuccessAlert, WarningAlert, ErrorAlert, InfoAlert};
```

## Usage

### Title Only

```rust
view! {
    #[template]
    SuccessAlert {
        #[template_child]
        title {
            set_label: "Changes saved successfully",
        },
    }
}
```

### Title + Description

```rust
view! {
    #[template]
    ErrorAlert {
        #[template_child]
        title {
            set_label: "Connection failed",
        },
        #[template_child]
        description {
            set_visible: true,
            set_label: "Unable to reach the server. Check your network connection.",
        },
    }
}
```

### Custom Icon

```rust
view! {
    #[template]
    WarningAlert {
        #[template_child]
        icon {
            set_icon_name: Some("battery-low-symbolic"),
        },
        #[template_child]
        title {
            set_label: "Battery low",
        },
    }
}
```

## Template Children

- **`icon`** - `gtk::Image`, status icon (defaults vary by type)
- **`title`** - `gtk::Label`, primary message
- **`description`** - `gtk::Label`, hidden by default
