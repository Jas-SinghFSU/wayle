# Button Templates

Widget templates for consistent button styling across Wayle.

## Available Templates

| Template | CSS Classes | Use Case |
|----------|-------------|----------|
| `PrimaryButton` | `.btn .btn-primary` | Main actions, CTAs |
| `SecondaryButton` | `.btn .btn-secondary` | Secondary actions |
| `DangerButton` | `.btn .btn-danger` | Destructive actions |
| `GhostButton` | `.btn .btn-ghost` | Subtle actions, toolbars |
| `GhostIconButton` | `.btn .btn-ghost-icon` | Icon-only ghost buttons |
| `IconButton` | `.btn .btn-icon` | Icon-only with background |
| `LinkButton` | `.btn-link` | Text links |
| `MutedLinkButton` | `.btn-link .muted` | De-emphasized links |
| `DangerLinkButton` | `.btn-link .danger` | Destructive text links |

## Import

```rust
use wayle_widgets::primitives::buttons::{PrimaryButton, SecondaryButton, GhostButton};
```

## Usage

### Label Only

```rust
view! {
    #[template]
    PrimaryButton {
        #[template_child]
        label {
            set_label: "Save Changes",
        },
    }
}
```

### Icon + Label

```rust
view! {
    #[template]
    PrimaryButton {
        #[template_child]
        icon {
            set_visible: true,
            set_icon_name: Some("document-save-symbolic"),
        },
        #[template_child]
        label {
            set_label: "Save",
        },
    }
}
```

### Icon Only

Use `IconButton` or `GhostIconButton` for icon-only buttons:

```rust
view! {
    #[template]
    GhostIconButton {
        set_icon_name: Some("window-close-symbolic"),
    }
}
```

## Template Children

Templates with text support expose two named children:

- **`icon`** - `gtk::Image`, hidden by default. Set `set_visible: true` to show.
- **`label`** - `gtk::Label` for button text.

`IconButton` and `GhostIconButton` have no children - use `set_icon_name`
directly on the button.

## Signal Handling

```rust
view! {
    #[template]
    PrimaryButton {
        connect_clicked => Msg::SaveClicked,
        #[template_child]
        label {
            set_label: "Save",
        },
    }
}
```

## Dynamic State

```rust
view! {
    #[template]
    PrimaryButton {
        #[watch]
        set_sensitive: !model.is_loading,
        #[template_child]
        label {
            #[watch]
            set_label: if model.is_loading { "Saving..." } else { "Save" },
        },
    }
}
```
