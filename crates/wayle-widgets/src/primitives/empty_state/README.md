# Empty State Template

Placeholder content when no data is available.

## Available

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
    }
}
```

## Template Children

- **`icon`** - `gtk::Image`, displays `tb-alert-triangle-symbolic` by default
- **`title`** - `gtk::Label` for the main heading
- **`description`** - `gtk::Label` for secondary text
