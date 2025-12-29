# Wayle

> **⚠️ Work in Progress**: Wayle is under active development. Core infrastructure is functional, but UI components and many services are not yet implemented. Not ready for daily use.

A fast, configurable desktop environment shell for Wayland compositors. Built in Rust with Relm4 and focused on performance, modularity, and a great user experience. A successor to HyprPanel without the pain or dependency on Hyprland.

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

- [ ] **Component Library (WIP)** - Base Relm4 widgets and containers
- [ ] **Bar Modules**:
  - [ ] Battery
  - [ ] Dashboard
  - [ ] Hyprland workspaces
  - [ ] Window title
  - [ ] Media
  - [ ] Notification
  - [ ] Volume
  - [ ] Network
  - [ ] Bluetooth
  - [ ] Clock
  - [ ] System tray
  - [ ] World clock
  - [ ] Separator
  - [ ] Microphone
  - [ ] RAM
  - [ ] CPU
  - [ ] CPU
  - [ ] Storage
  - [ ] Network
  - [ ] Keyboard input
  - [ ] Updates
  - [ ] Weather
  - [ ] Hyprland submap
  - [ ] Hyprsunset
  - [ ] Hypridle
  - [ ] Power

### Dropdown Interfaces

- [ ] **Audio Panel**
- [ ] **Network Panel**
- [ ] **Bluetooth Panel**
- [ ] **Media Panel**
- [ ] **Notifications Panel**
- [ ] **Calendar Panel**
- [ ] **Weather Panel**
- [ ] **Energy Panel**
- [ ] **Dashboard**

### Additional Features

- [ ] **Settings Dialog**
- [ ] **Notifications**
- [ ] **OSD**
- [ ] **Custom Modules**

## Configuration

Configuration is managed through TOML files, UI or CLI with live reloading and imports:

```toml
# main config.toml
imports = ["@themes/dark", "@modules/bar"]

[general]
theme = "dark"

# themes/dark.toml
[colors]
background = "#1e1e2e"
foreground = "#cdd6f4"

# modules/bar.toml
[bar]
position = "top"

[modules.clock]
format = "%H:%M"
```

Use the CLI to manage configuration:

```bash
# General help
wayle help

# Category help
wayle config help

# Get current config values
wayle config get modules.clock.format

# Set values with validation
wayle config set general.theme dark
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
cargo install --path wayle
```

## Icons

Wayle uses GTK symbolic icons that support CSS color theming. Bundled icons required by core components are installed automatically on first launch.

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
