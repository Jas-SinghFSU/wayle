# Spinner Template

Loading indicator with animated rotation.

## Available Templates

| Template | CSS Classes | Use Case |
|----------|-------------|----------|
| `Spinner` | `.spinner` | Indicate loading/processing state |

## Size Variants

| Class | Icon Size | Use Case |
|-------|-----------|----------|
| (default) | 2rem | Standard loading indicator |
| `.sm` | 1.5rem | Inline with text, compact UI |
| `.lg` | 2.5rem | Full-page or section loading |

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
        Spinner {},

        #[template]
        Spinner {
            add_css_class: "lg",
        },
    }
}
```

### In a Button

```rust
view! {
    #[template]
    PrimaryButton {
        #[template_child]
        icon {
            set_visible: false,
        },
        #[template_child]
        label {
            #[watch]
            set_visible: !model.is_loading,
            set_label: "Submit",
        },
        #[template]
        Spinner {
            #[watch]
            set_visible: model.is_loading,
            add_css_class: "sm",
        },
    }
}
```

## Dynamic State

The spinner is always spinning when visible. Control visibility to show/hide:

```rust
view! {
    #[template]
    Spinner {
        #[watch]
        set_visible: model.is_loading,
    }
}
```

## Icon Customization

The spinner uses `tb-loader-2-symbolic` from Tabler icons. To use a different icon, override in CSS:

```scss
.spinner.custom {
  // Assuming you've installed the icon via 'wayle icons install'
  -gtk-icon-source: -gtk-icontheme('tb-loader-symbolic');
}
```
