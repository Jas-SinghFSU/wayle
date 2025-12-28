# Slider Template

Widget template for range sliders with Wayle styling.

## Available Templates

| Template | CSS Classes | Use Case |
|----------|-------------|----------|
| `Slider` | `.slider` | Volume, brightness, progress controls |

## Import

```rust
use wayle_widgets::primitives::slider::Slider;
```

## Usage

### Basic

```rust
view! {
    #[template]
    Slider {
        set_range: (0.0, 100.0),
        set_value: 50.0,
    }
}
```

### With Custom Range

```rust
view! {
    #[template]
    Slider {
        set_range: (0.0, 1.0),
        set_value: 0.75,
    }
}
```

### Disabled

```rust
view! {
    #[template]
    Slider {
        set_range: (0.0, 100.0),
        set_value: 25.0,
        set_sensitive: false,
    }
}
```

## Signal Handling

Use `connect_value_changed` for value updates:

```rust
view! {
    #[template]
    Slider {
        set_range: (0.0, 100.0),
        set_value: 50.0,
        connect_value_changed[sender] => move |scale| {
            sender.input(Msg::VolumeChanged(scale.value()));
        },
    }
}
```

## Dynamic State

```rust
view! {
    #[template]
    Slider {
        set_range: (0.0, 100.0),
        #[watch]
        set_value: model.volume,
        #[watch]
        set_sensitive: !model.is_muted,
    }
}
```

## CSS Structure

```
scale.slider            /* Root container */
╰── trough              /* Track background */
    ├── highlight       /* Filled portion (origin to value) */
    ╰── slider          /* Draggable thumb */
```

### States

| Pseudo-class | Applies to |
|--------------|------------|
| `:hover` | Scale on mouse hover |
| `:disabled` | Entire slider when insensitive |
| `:focus-visible` | Scale on keyboard focus |

## Template Defaults

| Property | Value | Effect |
|----------|-------|--------|
| `set_draw_value` | `false` | Hides numeric value label |
| `set_has_origin` | `true` | Shows filled highlight from origin |
| `set_cursor_from_name` | `"pointer"` | Pointer cursor on hover |
