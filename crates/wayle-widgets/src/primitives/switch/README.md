# Switch Template

Toggle switch for binary on/off states.

## Available

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
    Switch {
        set_active: true,
    }
}
```

### With Signal

Use `connect_state_set` for toggle events. Return `glib::Propagation::Proceed` to allow the state change:

```rust
view! {
    #[template]
    Switch {
        #[watch]
        set_active: model.is_enabled,
        connect_state_set[sender] => move |_switch, state| {
            sender.input(Msg::Toggled(state));
            glib::Propagation::Proceed
        },
    }
}
```
