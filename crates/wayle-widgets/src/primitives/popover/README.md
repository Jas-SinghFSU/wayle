# Popover Components

Widget templates and factory component for popover menus with selectable items.

## Available Components

| Component | Type | CSS Classes | Use Case |
|-----------|------|-------------|----------|
| `Popover` | WidgetTemplate | `.popover` | Container for popover content |
| `PopoverHeader` | WidgetTemplate | `.popover-header` | Section header with label |
| `PopoverItem` | FactoryComponent | `.popover-item` | Selectable list item |

## Import

```rust
use wayle_widgets::primitives::popover::{Popover, PopoverHeader, PopoverItem};
use relm4::factory::FactoryVecDeque;
```

## Usage

### Basic Popover with Header

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

PopoverItem is a FactoryComponent; use with `FactoryVecDeque`:

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

let model = MyComponent { items };

let item_list = model.items.widget();
let widgets = view_output!();

ComponentParts { model, widgets }

// In view!:
#[local_ref]
item_list -> gtk::ListBox {
    add_css_class: "popover-list",
    set_selection_mode: gtk::SelectionMode::None,
},
```

### PopoverItem Fields

| Field | Type | Description |
|-------|------|-------------|
| `icon` | `Option<String>` | Leading icon name |
| `label` | `String` | Primary text |
| `subtitle` | `Option<String>` | Secondary text |
| `active_icon` | `Option<String>` | Selection indicator icon |
| `is_active` | `bool` | Show active indicator |

### Minimal Item

```rust
PopoverItem {
    icon: None,
    label: "Option".into(),
    subtitle: None,
    active_icon: None,
    is_active: false,
}
```

### Full Item

```rust
PopoverItem {
    icon: Some("audio-headphones-symbolic".into()),
    label: "USB Headset".into(),
    subtitle: Some("Logitech G Pro".into()),
    active_icon: Some("object-select-symbolic".into()),
    is_active: true,
}
```

## Template Children

### PopoverHeader

- **`label`** - `gtk::Label` for the header text

## Signal Handling

Handle item selection via ListBox's `row-activated` signal:

```rust
#[local_ref]
item_list -> gtk::ListBox {
    set_selection_mode: gtk::SelectionMode::None,
    connect_row_activated[sender] => move |_listbox, row| {
        let index = row.index() as usize;
        sender.input(Msg::ItemSelected(index));
    },
},
```

## CSS Classes

| Class | Element |
|-------|---------|
| `.popover` | Main popover container |
| `.popover-header` | Section header box |
| `.popover-header-label` | Header label text |
| `.popover-item` | ListBoxRow for each item |
| `.popover-item-content` | Horizontal content box |
| `.popover-item-icon-container` | Icon wrapper with background |
| `.popover-item-icon` | Icon image |
| `.popover-item-label-container` | Vertical box for labels |
| `.popover-item-label` | Primary label |
| `.popover-item-subtitle` | Secondary label |
| `.popover-item-active` | Active/selection indicator |
