# Dropdown Templates

Composable container templates for dropdown panels.

## Available

| Template          | CSS Classes         | Use Case                              |
| ----------------- | ------------------- | ------------------------------------- |
| `Dropdown`        | `.dropdown`         | Main container                        |
| `DropdownHeader`  | `.dropdown-header`  | Title bar with icon, label, actions   |
| `DropdownContent` | `.dropdown-content` | Main content area                     |
| `DropdownFooter`  | `.dropdown-footer`  | Footer for links or secondary actions |

## Import

```rust
use wayle_widgets::primitives::dropdown::{Dropdown, DropdownHeader, DropdownContent, DropdownFooter};
```

## Usage

### Content Only

```rust
view! {
    #[template]
    Dropdown {
        #[template]
        DropdownContent {
            gtk::Label {
                set_label: "Dropdown content here",
            },
        },
    }
}
```

### With Header

```rust
view! {
    #[template]
    Dropdown {
        #[template]
        DropdownHeader {
            #[template_child]
            icon {
                set_visible: true,
                set_icon_name: Some("tb-wifi-symbolic"),
            },
            #[template_child]
            label {
                set_label: "Wi-Fi",
            },
        },

        #[template]
        DropdownContent {
            // Network list here
        },
    }
}
```

### Full Configuration

```rust
view! {
    #[template]
    Dropdown {
        #[template]
        DropdownHeader {
            #[template_child]
            icon {
                set_visible: true,
                set_icon_name: Some("tb-wifi-symbolic"),
            },
            #[template_child]
            label {
                #[watch]
                set_label: &model.network_name,
            },
            #[template_child]
            actions {
                #[template]
                Switch {
                    #[watch]
                    set_active: model.wifi_enabled,
                    connect_state_set[sender] => move |_, state| {
                        sender.input(Msg::WifiToggled(state));
                        glib::Propagation::Proceed
                    },
                },
            },
        },

        #[template]
        DropdownContent {
            // Network list here
        },

        #[template]
        DropdownFooter {
            #[template]
            LinkButton {
                set_halign: gtk::Align::Center,
                set_hexpand: true,
                #[template_child]
                label {
                    set_label: "Wi-Fi Settings",
                },
            },
        },
    }
}
```

## Template Children

### DropdownHeader

- **`icon`** - `gtk::Image`, hidden by default
- **`label`** - `gtk::Label` for header title
- **`actions`** - `gtk::Box` for action widgets
