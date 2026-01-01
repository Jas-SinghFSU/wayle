# Card Template

Elevated container for grouping related content.

## Available

| Template | CSS Classes | Use Case                       |
| -------- | ----------- | ------------------------------ |
| `Card`   | `.card`     | Content grouping, data display |

## Import

```rust
use wayle_widgets::primitives::card::{Card, CardClass};
```

## Class Constants

| Constant              | CSS Class   | Effect             |
| --------------------- | ----------- | ------------------ |
| `CardClass::BORDERED` | `.bordered` | Adds subtle border |
| `CardClass::COMPACT`  | `.compact`  | Reduced padding    |
| `CardClass::SPACIOUS` | `.spacious` | Increased padding  |
| `CardClass::SHADOWED` | `.shadowed` | Adds drop shadow   |

## Usage

### Basic

```rust
view! {
    #[template]
    Card {
        gtk::Label {
            set_label: "Card content",
        },
    }
}
```

### With Modifiers

```rust
view! {
    #[template]
    Card {
        add_css_class: CardClass::BORDERED,
        add_css_class: CardClass::SHADOWED,
        gtk::Label {
            set_label: "Bordered card with shadow",
        },
    }
}
```

### Compact Card

```rust
view! {
    #[template]
    Card {
        add_css_class: CardClass::COMPACT,
        gtk::Label {
            set_label: "Compact content",
        },
    }
}
```

### Nested Cards

```rust
view! {
    #[template]
    Card {
        add_css_class: CardClass::SPACIOUS,
        #[template]
        Card {
            add_css_class: CardClass::BORDERED,
            gtk::Label {
                set_label: "Nested card",
            },
        },
    }
}
```
