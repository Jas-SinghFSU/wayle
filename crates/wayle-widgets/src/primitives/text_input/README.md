# Text Input Templates

Widget template for consistent text input styling across Wayle.

## Available Templates

| Template | CSS Classes | Use Case |
|----------|-------------|----------|
| `TextInput` | `.input` | General text entry |

## Variants via CSS Classes

Add these classes for state indication:

| Class | Effect |
|-------|--------|
| `.error` | Red focus ring for validation errors |
| `.warning` | Yellow focus ring for warnings |

## Import

```rust
use wayle_widgets::primitives::text_input::TextInput;
```

## Usage

### Basic

```rust
view! {
    #[template]
    TextInput {
        set_placeholder_text: Some("Enter text..."),
    }
}
```

### With Icons

```rust
view! {
    #[template]
    TextInput {
        set_placeholder_text: Some("Search..."),
        set_primary_icon_name: Some("system-search-symbolic"),
        set_secondary_icon_name: Some("edit-clear-symbolic"),
    }
}
```

### Password Field

```rust
view! {
    #[template]
    TextInput {
        set_placeholder_text: Some("Password"),
        set_visibility: false,
        set_invisible_char: Some('*'),
    }
}
```

### With Validation State

```rust
view! {
    #[template]
    TextInput {
        add_css_class: "error",
        set_placeholder_text: Some("Email"),
    }
}
```

## Common Properties

| Property | Type | Description |
|----------|------|-------------|
| `set_text` | `&str` | Set input value |
| `set_placeholder_text` | `Option<&str>` | Placeholder hint |
| `set_primary_icon_name` | `Option<&str>` | Left icon |
| `set_secondary_icon_name` | `Option<&str>` | Right icon |
| `set_visibility` | `bool` | Show/hide text (password) |
| `set_editable` | `bool` | Allow editing |
| `set_max_length` | `i32` | Character limit |
| `set_width_chars` | `i32` | Minimum width in chars |

## Signal Handling

```rust
view! {
    #[template]
    TextInput {
        set_placeholder_text: Some("Type here..."),
        connect_changed[sender] => move |entry| {
            sender.input(Msg::TextChanged(entry.text().to_string()));
        },
        connect_activate => Msg::EnterPressed,
        connect_icon_press[sender] => move |_, pos| {
            if pos == gtk::EntryIconPosition::Secondary {
                sender.input(Msg::ClearClicked);
            }
        },
    }
}
```

## Dynamic State

```rust
view! {
    #[template]
    TextInput {
        #[watch]
        set_text: &model.query,
        #[watch]
        set_sensitive: !model.is_loading,
        #[watch]
        add_css_class?: model.has_error.then_some("error"),
    }
}
```
