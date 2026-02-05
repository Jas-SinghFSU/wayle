# Wayle

> **⚠️ Work in Progress**: Wayle is under active development. The bar and
> modules that are completed under the [UI Components](#ui-components) section
> are ready to use. This, however, is not a stable environment, things are
> subject to change.

A fast, configurable desktop environment shell for Wayland compositors. Built in
Rust with Relm4 and focused on performance, modularity, and a great user
experience. A successor to HyprPanel without the pain or dependency on Hyprland.

## Progress

### Core Infrastructure

- [x] **Configuration System** - Reactive TOML config with schema validation
- [x] **CLI Interface** - Complete command-line management interface
- [x] **Documentation Generator** - Auto-generated config docs from schemas

### Services

- [x] **MPRIS**
- [x] **PulseAudio**
- [x] **Network**
- [x] **Bluetooth**
- [x] **Battery**
- [x] **Notification Daemon**
- [x] **Power Profiles**
- [x] **System Tray**
    - [x] GTK4 Adapter
- [x] Hyprland
- [x] **Cava**

### UI Components

- [x] **Component Library** - Base Relm4 widgets and containers
- [ ] **Bar Modules (WIP)**:
    - [x] Battery
    - [x] Media
    - [x] Volume
    - [x] Network
    - [x] Bluetooth
    - [x] Clock
    - [x] Microphone
    - [x] System tray
    - [x] Notification
    - [x] Dashboard
    - [x] Netstat
    - [x] RAM
    - [x] CPU
    - [x] CPU Temp
    - [x] Storage
    - [x] Separator
    - [x] Power
    - [x] World clock
    - [x] Weather
    - [x] Idle Inhibit
    - [x] Keyboard input
    - [x] Hyprland Window title
    - [x] Hyprland submap
    - [ ] Hyprsunset
    - [ ] Hyprland workspaces
    - [ ] Custom Modules

#### Scoped out

- **Updates**
    - Too much surface area and distro coupling
    - Will be achievable easily via custom modules

### Dropdown Interfaces

- [ ] **Audio Panel**
- [ ] **Battery Panel**
- [ ] **Bluetooth Panel**
- [ ] **Calendar Panel**
- [ ] **Dashboard**
- [ ] **Media Panel**
- [ ] **Network Panel**
- [ ] **Notifications Panel**
- [ ] **Weather Panel**

### Additional Features

- [ ] **Settings Dialog**
- [ ] **Notifications**
- [ ] **OSD**
- [ ] **Custom Modules**

## Configuration

Configuration lives in `~/.config/wayle/config.toml` with live reloading.

```toml
[styling]
theme-provider = "wayle"

[styling.palette]
bg = "#16161e"
fg = "#c0caf5"
primary = "#7aa2f7"

[bar]
scale = 1
location = "top"
rounding = "sm"

[[bar.layout]]
monitor = "*"
left = ["clock"]
center = ["media"]
right = ["battery"]

[modules.clock]
format = "%H:%M"
icon-show = true
label-show = true
```

Config files can be split and imported for better organization:

```toml
# config.toml
imports = ["@colors.toml", "@modules/bar.toml"]

[bar]
location = "top"
```

Paths prefixed with `@` resolve relative to the config directory.

CLI commands can also be used to modify, get or reset any property:

```bash
wayle config get bar.scale
wayle config set bar.location bottom
wayle config reset bar.scale
```

Editor intellisense is available via JSON Schema. Install
[Tombi](https://marketplace.visualstudio.com/items?itemName=tombi-toml.tombi)
for VSCode or the `tombi` LSP for Neovim. The schema is generated automatically
on startup.

This will give you auto-complete, config validation and other nice QoL features
for your config.toml (and other toml files).

```bash
wayle config schema
```

## Building

Switch to nightly Rust:

```bash
rustup toolchain install nightly
rustup default nightly
```

Then clone the repository and build:

```bash
git clone https://github.com/Jas-SinghFSU/wayle
cd wayle
cargo install --path crates/wayle-shell
cargo install --path wayle
```

Once Wayle is installed, you can set up the icons (temporary measure) and start
it via:

```bash
wayle icons setup
wayle panel start
```

## Icons

Wayle uses GTK symbolic icons that support CSS color theming.

To manually manage icons:

```bash
# Install bundled icons (automatic on first launch)
wayle icons setup

# Install additional icons from CDN sources
wayle icons install tabler home settings bell
wayle icons install simple-icons firefox spotify

# See all available sources
wayle icons install --help
```

Icons are installed to `~/.local/share/wayle/icons/` as GTK symbolic icons.

## License

MIT
