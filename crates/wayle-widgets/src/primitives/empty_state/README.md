# Empty State Template

Widget template for displaying placeholder content when no data is available.

## Available Templates

| Template | CSS Classes | Use Case |
|----------|-------------|----------|
| `EmptyState` | `.empty-state` | Empty lists, no results, error states |

## Import

```rust
use wayle_widgets::primitives::empty_state::EmptyState;
```

## Usage

### Basic

```rust
view! {
    #[template]
    EmptyState {
        #[template_child]
        title {
            set_label: "No devices found",
        },
    }
}
```

### With Description

```rust
view! {
    #[template]
    EmptyState {
        #[template_child]
        title {
            set_label: "No devices found",
        },
        #[template_child]
        description {
            set_label: "Connect a device to get started",
        },
    }
}
```

### Custom Icon

```rust
view! {
    #[template]
    EmptyState {
        #[template_child]
        icon {
            set_icon_name: Some("tb-wifi-off-symbolic"),
        },
        #[template_child]
        title {
            set_label: "No network connection",
        },
        #[template_child]
        description {
            set_label: "Check your network settings",
        },
    }
}
```

## Template Children

- **`icon`** - `gtk::Image`, displays `tb-alert-triangle-symbolic` by default.
- **`title`** - `gtk::Label` for the main heading.
- **`description`** - `gtk::Label` for secondary text.

## Icon Sizes

Add size classes to the icon for different scales:

```rust
#[template_child]
icon {
    add_css_class: "sm",  // Smaller icon (--icon-2xl)
}
```

| Class | Token | Use Case |
|-------|-------|----------|
| (none) | `--icon-4xl` | Default size |
| `.sm` | `--icon-2xl` | Compact empty states |
| `.lg` | `--icon-6xl` | Full-page empty states |

## Dynamic State

```rust
view! {
    #[template]
    EmptyState {
        #[template_child]
        icon {
            #[watch]
            set_icon_name: Some(if model.is_error {
                "tb-alert-circle-symbolic"
            } else {
                "tb-inbox-symbolic"
            }),
        },
        #[template_child]
        title {
            #[watch]
            set_label: if model.is_error { "Something went wrong" } else { "No items" },
        },
    }
}
```
