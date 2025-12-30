# ConfirmModal

Confirmation dialog for destructive or irreversible actions.

## Exports

| Export | Type | Purpose |
|--------|------|---------|
| `ConfirmModal` | Component | The modal dialog component |
| `ConfirmModalConfig` | Struct | Configuration for displaying the modal |
| `ConfirmModalMsg` | Enum | Input messages (`Show`, `Hide`, `Confirm`, `Cancel`) |
| `ConfirmModalOutput` | Enum | Output messages (`Confirmed`, `Cancelled`) |
| `ModalIcon` | Enum | Icon variants (`Warning`, `Error`, `Success`, `Info`, `None`) |
| `ConfirmStyle` | Enum | Confirm button style (`Danger`, `Primary`) |

## Import

```rust
use wayle_widgets::primitives::confirm_modal::{
    ConfirmModal, ConfirmModalConfig, ConfirmModalMsg, ConfirmModalOutput,
    ConfirmStyle, ModalIcon,
};
```

## Usage

Create the modal once at component init, store the `Controller`, then send `Show(config)` messages to display.

### Setup

```rust
struct App {
    modal: Controller<ConfirmModal>,
}

fn init(
    _init: Self::Init,
    root: Self::Root,
    sender: ComponentSender<Self>,
) -> ComponentParts<Self> {
    let modal = ConfirmModal::builder()
        .transient_for(&root)
        .launch(())
        .forward(sender.input_sender(), Msg::ModalResult);

    let model = App { modal };
    let widgets = view_output!();
    ComponentParts { model, widgets }
}
```

### Showing the Modal

```rust
self.modal.emit(ConfirmModalMsg::Show(ConfirmModalConfig {
    title: "Delete Account?".into(),
    description: Some("This will permanently delete your account.".into()),
    icon: ModalIcon::Warning,
    confirm_label: "Delete".into(),
    confirm_style: ConfirmStyle::Danger,
    cancel_label: None,
}));
```

### Handling Output

```rust
fn update(&mut self, msg: Msg, _sender: ComponentSender<Self>) {
    match msg {
        Msg::ModalResult(output) => match output {
            ConfirmModalOutput::Confirmed => {
                // User confirmed the action
            }
            ConfirmModalOutput::Cancelled => {
                // User cancelled (button, ESC, or window close)
            }
        },
    }
}
```

## ConfirmModalConfig Fields

| Field | Type | Purpose |
|-------|------|---------|
| `title` | `String` | Modal title |
| `description` | `Option<String>` | Explanatory text below title |
| `icon` | `ModalIcon` | Header icon style |
| `confirm_label` | `String` | Confirm button text |
| `confirm_style` | `ConfirmStyle` | Confirm button styling (`Danger` or `Primary`) |
| `cancel_label` | `Option<String>` | Cancel button text (default: "Cancel") |

## Icon Variants

| Variant | Icon | Background | Use Case |
|---------|------|------------|----------|
| `Warning` | `tb-alert-triangle-symbolic` | `--status-warning-subtle` | Destructive actions |
| `Error` | `tb-x-symbolic` | `--status-error-subtle` | Error confirmations |
| `Success` | `tb-check-symbolic` | `--status-success-subtle` | Success confirmations |
| `Info` | `tb-info-circle-symbolic` | `--accent-subtle` | Informational prompts |
| `None` | (hidden) | - | No icon |

## CSS Classes

| Class | Element |
|-------|---------|
| `.modal` | Root window |
| `.modal-header` | Header container |
| `.modal-icon` | Icon container (+ `.warning`, `.error`, `.success`, `.info`) |
| `.modal-header-content` | Title/description container |
| `.modal-title` | Title label |
| `.modal-description` | Description label |
| `.modal-footer` | Button container |

## Keyboard

- **ESC** - Cancels and closes the modal
