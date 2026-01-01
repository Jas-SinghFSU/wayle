# Progress Bar Template

Linear progress indicator for determinate progress.

## Available

| Template      | CSS Classes     | Use Case                          |
| ------------- | --------------- | --------------------------------- |
| `ProgressBar` | `.progress-bar` | Battery, brightness/volume levels |

### Status Variants

| Class      | Fill Color         | Use Case             |
| ---------- | ------------------ | -------------------- |
| (default)  | `--accent`         | Standard progress    |
| `.success` | `--status-success` | Completion indicator |
| `.warning` | `--status-warning` | Caution state        |
| `.error`   | `--status-error`   | Critical             |

## Import

```rust
use wayle_widgets::primitives::progress_bar::{ProgressBar, ProgressBarClass};
```

## Class Constants

### Status

| Constant                    | CSS Class  | Effect      |
| --------------------------- | ---------- | ----------- |
| `ProgressBarClass::SUCCESS` | `.success` | Green fill  |
| `ProgressBarClass::WARNING` | `.warning` | Yellow fill |
| `ProgressBarClass::ERROR`   | `.error`   | Red fill    |

### Size

| Constant                  | CSS Class | Effect       |
| ------------------------- | --------- | ------------ |
| `ProgressBarClass::SMALL` | `.sm`     | Compact size |
| `ProgressBarClass::LARGE` | `.lg`     | Large size   |

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
        add_css_class: ProgressBarClass::ERROR,
        set_fraction: 0.15,
    }
}
```

### Size Variants

```rust
view! {
    #[template]
    ProgressBar {
        add_css_class: ProgressBarClass::SMALL,
        add_css_class: ProgressBarClass::SUCCESS,
        set_fraction: 1.0,
    }

    #[template]
    ProgressBar {
        add_css_class: ProgressBarClass::LARGE,
        set_fraction: 0.65,
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
