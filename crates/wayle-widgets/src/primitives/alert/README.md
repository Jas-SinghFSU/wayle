# Alert Templates

Widget templates for status messages and notifications.

## Available Templates

| Template | CSS Classes | Use Case |
|----------|-------------|----------|
| `Alert` | `.alert` | Neutral status messages |
| `SuccessAlert` | `.alert .success` | Positive confirmations |
| `WarningAlert` | `.alert .warning` | Caution messages |
| `ErrorAlert` | `.alert .error` | Failure messages |
| `InfoAlert` | `.alert .info` | Informational messages |

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

All alert templates expose three named children:

- **`icon`** - `gtk::Image`, displays status icon. Default icons vary by template.
- **`title`** - `gtk::Label`, primary message text.
- **`description`** - `gtk::Label`, hidden by default. Set `set_visible: true` to show.

### Default Icons

| Template | Default Icon |
|----------|--------------|
| `Alert` | `dialog-information-symbolic` |
| `SuccessAlert` | `emblem-ok-symbolic` |
| `WarningAlert` | `dialog-warning-symbolic` |
| `ErrorAlert` | `dialog-error-symbolic` |
| `InfoAlert` | `dialog-information-symbolic` |

## Dynamic State

```rust
view! {
    #[template]
    InfoAlert {
        #[template_child]
        title {
            #[watch]
            set_label: &model.status_message,
        },
        #[template_child]
        description {
            #[watch]
            set_visible: model.show_details,
            #[watch]
            set_label: &model.details,
        },
    }
}
```

## Conditional Alert Type

```rust
view! {
    gtk::Box {
        if model.has_error {
            #[template]
            ErrorAlert {
                #[template_child]
                title {
                    #[watch]
                    set_label: &model.error_message,
                },
            }
        } else if model.has_warning {
            #[template]
            WarningAlert {
                #[template_child]
                title {
                    #[watch]
                    set_label: &model.warning_message,
                },
            }
        }
    }
}
```
