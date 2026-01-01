# Text Input Template

Text entry field with Wayle styling.

## Available

| Template    | CSS Classes | Use Case           |
| ----------- | ----------- | ------------------ |
| `TextInput` | `.input`    | General text entry |

### State Classes

| Class      | Effect                               |
| ---------- | ------------------------------------ |
| `.error`   | Red focus ring for validation errors |
| `.warning` | Yellow focus ring for warnings       |

## Import

```rust
use wayle_widgets::primitives::text_input::{TextInput, TextInputClass};
```

## Class Constants

| Constant                | CSS Class  | Effect                   |
| ----------------------- | ---------- | ------------------------ |
| `TextInputClass::ERROR` | `.error`   | Red focus ring for error |
| `TextInputClass::WARNING` | `.warning` | Yellow focus ring        |

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

### With Validation State

```rust
view! {
    #[template]
    TextInput {
        add_css_class: TextInputClass::ERROR,
        set_placeholder_text: Some("Invalid input"),
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
    }
}
```

### With Signal

```rust
view! {
    #[template]
    TextInput {
        #[watch]
        set_text: &model.query,
        connect_changed[sender] => move |entry| {
            sender.input(Msg::TextChanged(entry.text().to_string()));
        },
        connect_activate => Msg::EnterPressed,
    }
}
```
