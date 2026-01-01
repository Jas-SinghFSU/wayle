# Separator Templates

Visual dividers for separating content sections.

## Available

| Template              | CSS Classes  | Use Case                      |
| --------------------- | ------------ | ----------------------------- |
| `HorizontalSeparator` | `.separator` | Dividing stacked content      |
| `VerticalSeparator`   | `.separator` | Dividing side-by-side content |

## Import

```rust
use wayle_widgets::primitives::separator::{HorizontalSeparator, VerticalSeparator};
```

## Usage

### Horizontal

```rust
view! {
    gtk::Box {
        set_orientation: gtk::Orientation::Vertical,

        gtk::Label { set_label: "Section 1" },

        #[template]
        HorizontalSeparator {},

        gtk::Label { set_label: "Section 2" },
    }
}
```

### Vertical

```rust
view! {
    gtk::Box {
        gtk::Label { set_label: "Left" },

        #[template]
        VerticalSeparator {},

        gtk::Label { set_label: "Right" },
    }
}
```
