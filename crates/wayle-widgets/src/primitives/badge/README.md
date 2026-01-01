# Badge Templates

Status indicators, labels, and tags.

## Available

### Filled Badges

| Template       | CSS Classes       | Use Case                     |
| -------------- | ----------------- | ---------------------------- |
| `Badge`        | `.badge`          | Default accent-colored badge |
| `SuccessBadge` | `.badge .success` | Positive status, completion  |
| `WarningBadge` | `.badge .warning` | Caution, pending states      |
| `ErrorBadge`   | `.badge .error`   | Errors, critical alerts      |
| `InfoBadge`    | `.badge .info`    | Informational, neutral       |

### Subtle Badges

| Template             | CSS Classes              | Use Case             |
| -------------------- | ------------------------ | -------------------- |
| `SubtleBadge`        | `.badge-subtle`          | Softer accent badge  |
| `SubtleSuccessBadge` | `.badge-subtle .success` | Soft positive status |
| `SubtleWarningBadge` | `.badge-subtle .warning` | Soft warning         |
| `SubtleErrorBadge`   | `.badge-subtle .error`   | Soft error indicator |
| `SubtleInfoBadge`    | `.badge-subtle .info`    | Soft informational   |

## Import

```rust
use wayle_widgets::primitives::badge::{Badge, SuccessBadge, ErrorBadge, SubtleBadge};
```

## Usage

### Basic

```rust
view! {
    #[template]
    Badge {
        set_label: "New",
    }
}
```

### Status Indicators

```rust
view! {
    gtk::Box {
        set_spacing: 8,

        #[template]
        SuccessBadge {
            set_label: "Online",
        },

        #[template]
        ErrorBadge {
            set_label: "Offline",
        },
    }
}
```

### Dynamic

```rust
view! {
    #[template]
    SuccessBadge {
        #[watch]
        set_label: &model.count.to_string(),
    }
}
```
