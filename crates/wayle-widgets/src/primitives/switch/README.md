# Switch Template

Widget template for toggle switches with Wayle styling.

## Available Templates

| Template | CSS Classes | Use Case |
|----------|-------------|----------|
| `Switch` | `.switch` | Binary on/off toggles |

## Import

```rust
use wayle_widgets::primitives::switch::Switch;
```

## Usage

### Basic

```rust
view! {
    #[template]
    Switch {}
}
```

### Pre-checked

```rust
view! {
    #[template]
    Switch {
        set_active: true,
    }
}
```

### Disabled

```rust
view! {
    #[template]
    Switch {
        set_sensitive: false,
    }
}
```

## Signal Handling

Use `connect_state_set` for toggle events. Return `glib::Propagation::Proceed` to allow the state change:

```rust
view! {
    #[template]
    Switch {
        connect_state_set[sender] => move |_switch, state| {
            sender.input(Msg::Toggled(state));
            glib::Propagation::Proceed
        },
    }
}
```

## Dynamic State

```rust
view! {
    #[template]
    Switch {
        #[watch]
        set_active: model.is_enabled,
        #[watch]
        set_sensitive: !model.is_loading,
    }
}
```

## CSS Structure

```
switch                  /* Track */
├── image              /* On icon (hidden by default) */
├── image              /* Off icon (hidden by default) */
╰── slider             /* Puck/thumb */
```

### States

| Pseudo-class | Applies to |
|--------------|------------|
| `:checked` | Track when active |
| `:disabled` | Entire switch when insensitive |
| `:hover` | Track on mouse hover |
| `:focus` | Track on keyboard focus |
