# Spinner Template

Loading indicator with animated rotation.

## Available

| Template  | CSS Classes | Use Case                 |
| --------- | ----------- | ------------------------ |
| `Spinner` | `.spinner`  | Loading/processing state |

### Size Variants

| Class     | Size   | Use Case           |
| --------- | ------ | ------------------ |
| (default) | 2rem   | Standard indicator |
| `.sm`     | 1.5rem | Inline, compact UI |
| `.lg`     | 2.5rem | Full-page loading  |

## Import

```rust
use wayle_widgets::primitives::spinner::{Spinner, SpinnerClass};
```

## Usage

### Basic

```rust
view! {
    #[template]
    Spinner {}
}
```

### Size Variants

```rust
view! {
    gtk::Box {
        #[template]
        Spinner {
            add_css_class: SpinnerClass::SM,
        },

        #[template]
        Spinner {
            add_css_class: SpinnerClass::LG,
        },
    }
}
```

## Class Constants

| Constant           | CSS Class | Effect     |
| ------------------ | --------- | ---------- |
| `SpinnerClass::SM` | `.sm`     | Small size |
| `SpinnerClass::LG` | `.lg`     | Large size |

### Conditional Display

```rust
view! {
    #[template]
    Spinner {
        #[watch]
        set_visible: model.is_loading,
    }
}
```
