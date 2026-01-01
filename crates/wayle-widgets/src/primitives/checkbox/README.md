# Checkbox Template

Checkbox for multi-select options.

## Available

| Template   | CSS Classes | Use Case                              |
| ---------- | ----------- | ------------------------------------- |
| `Checkbox` | `.checkbox` | Settings, multi-select, feature flags |

## Import

```rust
use wayle_widgets::primitives::checkbox::{Checkbox, CheckboxClass};
```

## Usage

### Basic

```rust
view! {
    #[template]
    Checkbox {
        set_label: Some("Enable notifications"),
    }
}
```

### Checked State

```rust
view! {
    #[template]
    Checkbox {
        set_label: Some("I agree to terms"),
        set_active: true,
    }
}
```

### Dynamic State

```rust
view! {
    #[template]
    Checkbox {
        set_label: Some("Auto-save"),
        #[watch]
        set_active: model.auto_save_enabled,
        connect_toggled[sender] => move |checkbox| {
            sender.input(Msg::AutoSaveToggled(checkbox.is_active()));
        },
    }
}
```

### Without Label

```rust
view! {
    #[template]
    Checkbox {
        set_active: true,
    }
}
```

### Indeterminate State

```rust
view! {
    #[template]
    Checkbox {
        set_label: Some("Select all"),
        set_inconsistent: true,
    }
}
```

## GTK Widget

| Property  | Value                                          |
| --------- | ---------------------------------------------- |
| Root      | `gtk::CheckButton`                             |
| CSS Node  | `checkbutton`                                  |
| Indicator | `check` CSS node (gets `:checked` when active) |
| States    | `:checked`, `:indeterminate`, `:disabled`      |

## CSS Styling

```css
/* The indicator box */
checkbutton.checkbox check {
    min-width: 1.125rem;
    min-height: 1.125rem;
    border: 2px solid var(--fg-muted);
    border-radius: var(--radius-sm);
}

/* Checked state */
checkbutton.checkbox check:checked {
    background: var(--accent);
    border-color: var(--accent);
}

/* Indeterminate state */
checkbutton.checkbox check:indeterminate {
    background: var(--accent);
    border-color: var(--accent);
}
```

## Radio Buttons

To create radio buttons (single-select), group multiple CheckButtons together:

```rust
fn init(...) -> ComponentParts<Self> {
    let widgets = view_output!();

    // Group radio buttons - only one can be active
    widgets.option_b.set_group(Some(&widgets.option_a));
    widgets.option_c.set_group(Some(&widgets.option_a));

    ComponentParts { model, widgets }
}

view! {
    gtk::Box {
        set_orientation: gtk::Orientation::Vertical,

        #[name = "option_a"]
        #[template]
        Checkbox {
            set_label: Some("Option A"),
            set_active: true,
        },
        #[name = "option_b"]
        #[template]
        Checkbox {
            set_label: Some("Option B"),
        },
        #[name = "option_c"]
        #[template]
        Checkbox {
            set_label: Some("Option C"),
        },
    }
}
```

When grouped, the CSS node changes from `check` to `radio`, and the indicator
becomes circular.
