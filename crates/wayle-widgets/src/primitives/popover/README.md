# Popover Components

Templates and factory component for popover menus.

## Available

| Component       | Type             | Use Case                      |
| --------------- | ---------------- | ----------------------------- |
| `Popover`       | Template         | Container for popover content |
| `PopoverHeader` | Template         | Section header with label     |
| `PopoverItem`   | FactoryComponent | Selectable list item          |

## Import

```rust
use wayle_widgets::primitives::popover::{Popover, PopoverHeader, PopoverItem};
use relm4::factory::FactoryVecDeque;
```

## Usage

### Basic Popover

```rust
view! {
    gtk::MenuButton {
        set_label: "Open",

        #[wrap(Some)]
        #[template]
        set_popover = &Popover {
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                #[template]
                PopoverHeader {
                    #[template_child]
                    label {
                        set_label: "Output Devices",
                    },
                },

                // Content goes here
            }
        },
    }
}
```

### PopoverItem with FactoryVecDeque

PopoverItem is a FactoryComponent for dynamic lists:

```rust
struct MyComponent {
    items: FactoryVecDeque<PopoverItem>,
}

// In init():
let mut items: FactoryVecDeque<PopoverItem> = FactoryVecDeque::builder()
    .launch(gtk::ListBox::default())
    .detach();

{
    let mut guard = items.guard();
    guard.push_back(PopoverItem {
        icon: Some("audio-card-symbolic".into()),
        label: "Built-in Audio".into(),
        subtitle: Some("HDA Intel PCH".into()),
        active_icon: Some("object-select-symbolic".into()),
        is_active: true,
    });
}

// In view!:
#[local_ref]
item_list -> gtk::ListBox {
    add_css_class: "popover-list",
    set_selection_mode: gtk::SelectionMode::None,
    connect_row_activated[sender] => move |_, row| {
        sender.input(Msg::ItemSelected(row.index() as usize));
    },
},
```

### PopoverItem Fields

| Field         | Type             | Description              |
| ------------- | ---------------- | ------------------------ |
| `icon`        | `Option<String>` | Leading icon name        |
| `label`       | `String`         | Primary text             |
| `subtitle`    | `Option<String>` | Secondary text           |
| `active_icon` | `Option<String>` | Selection indicator icon |
| `is_active`   | `bool`           | Show active indicator    |
