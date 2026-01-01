# Progress Bar Template

Linear progress indicator for determinate progress.

## Available

| Template | CSS Classes | Use Case |
|----------|-------------|----------|
| `ProgressBar` | `.progress-bar` | Battery, brightness/volume levels |

### Status Variants

| Class | Fill Color | Use Case |
|-------|------------|----------|
| (default) | `--accent` | Standard progress |
| `.success` | `--status-success` | Completion indicator |
| `.warning` | `--status-warning` | Caution state |
| `.error` | `--status-error` | Critical |

## Import

```rust
use wayle_widgets::primitives::progress_bar::ProgressBar;
```

## Usage

### Basic

```rust
view! {
    #[template]
    ProgressBar {
        set_fraction: 0.65,
    }
}
```

### With Status

```rust
view! {
    #[template]
    ProgressBar {
        add_css_class: "error",
        set_fraction: 0.15,
    }
}
```

### Dynamic

```rust
view! {
    #[template]
    ProgressBar {
        #[watch]
        set_fraction: model.progress,
    }
}
```
