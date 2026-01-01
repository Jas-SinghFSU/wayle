# Spinner Template

Loading indicator with animated rotation.

## Available

| Template | CSS Classes | Use Case |
|----------|-------------|----------|
| `Spinner` | `.spinner` | Loading/processing state |

### Size Variants

| Class | Size | Use Case |
|-------|------|----------|
| (default) | 2rem | Standard indicator |
| `.sm` | 1.5rem | Inline, compact UI |
| `.lg` | 2.5rem | Full-page loading |

## Import

```rust
use wayle_widgets::primitives::spinner::Spinner;
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
            add_css_class: "sm",
        },

        #[template]
        Spinner {
            add_css_class: "lg",
        },
    }
}
```

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
