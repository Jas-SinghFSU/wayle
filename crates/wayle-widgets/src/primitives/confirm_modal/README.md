# ConfirmModal

Confirmation dialog for destructive or irreversible actions.

## Available

| Export | Type | Purpose |
|--------|------|---------|
| `ConfirmModal` | Component | The modal dialog |
| `ConfirmModalConfig` | Struct | Display configuration |
| `ConfirmModalMsg` | Enum | Input: `Show`, `Hide`, `Confirm`, `Cancel` |
| `ConfirmModalOutput` | Enum | Output: `Confirmed`, `Cancelled` |
| `ModalIcon` | Enum | `Warning`, `Error`, `Success`, `Info`, `None` |
| `ConfirmStyle` | Enum | `Danger`, `Primary` |

## Import

```rust
use wayle_widgets::primitives::confirm_modal::{
    ConfirmModal, ConfirmModalConfig, ConfirmModalMsg, ConfirmModalOutput,
    ConfirmStyle, ModalIcon,
};
```

## Usage

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
                // User confirmed
            }
            ConfirmModalOutput::Cancelled => {
                // User cancelled
            }
        },
    }
}
```

## ConfirmModalConfig Fields

| Field | Type | Purpose |
|-------|------|---------|
| `title` | `String` | Modal title |
| `description` | `Option<String>` | Explanatory text |
| `icon` | `ModalIcon` | Header icon style |
| `confirm_label` | `String` | Confirm button text |
| `confirm_style` | `ConfirmStyle` | `Danger` or `Primary` |
| `cancel_label` | `Option<String>` | Cancel button text (default: "Cancel") |
