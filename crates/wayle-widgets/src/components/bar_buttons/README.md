# Bar Button

Configurable button component for shell panels with three visual variants.

## Variants

| Variant       | CSS Classes                | Description                               |
| ------------- | -------------------------- | ----------------------------------------- |
| `Basic`       | `.bar-button.basic`        | Icon and label, transparent background    |
| `BlockPrefix` | `.bar-button.block-prefix` | Colored icon block flush with button edge |
| `IconSquare`  | `.bar-button.icon-square`  | Colored icon square with button padding   |

## Import

```rust
use wayle_widgets::components::bar_buttons::{
    BarButton, BarButtonInit, BarButtonInput, BarButtonOutput,
    BarButtonVariant, BarButtonVariantConfig,
    BasicBarButtonConfig, BlockPrefixBarButtonConfig, IconSquareBarButtonConfig,
};
```

## Usage

### Basic

```rust
let bar_button = BarButton::builder()
    .launch(BarButtonInit {
        icon: "audio-volume-high-symbolic".into(),
        label: "100%".into(),
        ..Default::default()
    })
    .forward(sender.input_sender(), |output| match output {
        BarButtonOutput::LeftClick => Msg::ToggleMute,
        BarButtonOutput::ScrollUp => Msg::VolumeUp,
        BarButtonOutput::ScrollDown => Msg::VolumeDown,
        _ => Msg::Noop,
    });
```

### With Variant

```rust
BarButtonInit {
    icon: "network-wireless-symbolic".into(),
    label: "WiFi".into(),
    variant: BarButtonVariant::BlockPrefix,
    variant_config: BarButtonVariantConfig::BlockPrefix(
        BlockPrefixBarButtonConfig::default()
    ),
    ..Default::default()
}
```

### Runtime Updates

```rust
bar_button.emit(BarButtonInput::SetIcon("audio-volume-muted-symbolic".into()));
bar_button.emit(BarButtonInput::SetLabel("Muted".into()));
bar_button.emit(BarButtonInput::SetTooltip(Some("Click to unmute".into())));
```

### Variant Switching

```rust
bar_button.emit(BarButtonInput::SetVariant(
    BarButtonVariant::IconSquare,
    BarButtonVariantConfig::IconSquare(IconSquareBarButtonConfig::default()),
));
```

## Output Events

| Event         | Trigger             |
| ------------- | ------------------- |
| `LeftClick`   | Left mouse button   |
| `RightClick`  | Right mouse button  |
| `MiddleClick` | Middle mouse button |
| `ScrollUp`    | Scroll wheel up     |
| `ScrollDown`  | Scroll wheel down   |

## Configuration

All variants share these config properties:

| Property          | Type          | Default | Description                                 |
| ----------------- | ------------- | ------- | ------------------------------------------- |
| `show_label`      | `bool`        | `true`  | Show/hide label                             |
| `visible`         | `bool`        | `true`  | Show/hide entire button                     |
| `vertical`        | `bool`        | `false` | Vertical orientation                        |
| `label_max_chars` | `Option<u32>` | `None`  | Max chars before truncation (None=disabled) |
| `icon_color`      | `ColorValue`  | varies  | Icon foreground color                       |
| `label_color`     | `ColorValue`  | varies  | Label text color                            |

Config properties are reactive - changes trigger automatic UI updates.

## CSS Classes

| Class                                       | Applied When         |
| ------------------------------------------- | -------------------- |
| `.bar-button`                               | Always (base)        |
| `.basic` / `.block-prefix` / `.icon-square` | Per variant          |
| `.icon-only`                                | Label hidden         |
| `.vertical`                                 | Vertical orientation |
