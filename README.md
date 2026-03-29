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
- [x] **Bar Modules**:
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
  - [x] Hyprsunset
  - [x] Hyprland workspaces
  - [x] Custom Modules
  - [x] Cava

#### Scoped out

- **Updates**
  - Too much surface area and distro coupling
  - Will be achievable easily via custom modules

### Dropdown Interfaces

- [x] **Audio Panel**
- [x] **Network Panel**
- [x] **Bluetooth Panel**
- [x] **Battery Panel**
- [x] **Media Panel**
- [x] **Weather Panel**
- [x] **Calendar Panel**
- [x] **Dashboard**
- [x] **Notifications Panel**

### Additional Features

- [x] **Notifications**
- [x] **OSD**
- [ ] **Settings Dialog (WIP)**

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
imports = ["colors.toml", "modules/bar.toml"]

[bar]
location = "top"
```

CLI commands can also be used to modify, get or reset any property:

```bash
wayle config get bar.scale
wayle config set bar.location bottom
wayle config reset bar.scale
```

Once the project is finished, documentation will be added for all configurable
properties, in addition to having a settings GUI. Until then you can run the
following command to generate a reference config `config.toml.example` in your
config directory:

```bash
wayle config default
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

Install Rust via [rustup](https://rustup.rs):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the repository recursively and build:

```bash
git clone --recursive https://github.com/wayle-rs/wayle
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

## Installation

<details>
    <summary> <h3> NixOS - Flake </h3> </summary>

For systems running NixOS, wayle is available to be used as home-manager module
via a flake.

First, add wayle to your inputs in your `flake.nix` file:

```nix
# flake.nix
{
    # ...
    inputs = {
        # ...
        wayle = {
          url = "git+https://github.com/Jas-SinghFSU/wayle";
          inputs.nixpkgs.follows = "nixpkgs-unstable"; # Optional (not tested on stable nixpkgs)
        };
        # ...
    };
    # ...
}
```

Note, you must use the path `git+https://github.com/...` since this project uses
git submodules and the `github://...` path style does not support these.

Then, still in your `flake.nix` file, add wayle as a module to your home manager
config. How your home-manager is configured might be slightly different but in
this example home-manager is configured as a nix-module. The important part is
adding `inputs.wayle.homeManagerModules.default` to your home-manager shared
modules list.

```nix
# flake.nix
{
    # ...

    outputs = { nixpkgs, ... } @ inputs: {
        # ...
        nixosConfigurations.default = nixpkgs.lib.nixosSystem {
            # ...
            modules = [
                # ...
                inputs.home-manager.nixosModules.home-manager
                {
                    home-manager = {
                        # ...

                        sharedModules = [
                            # ...

                            # Adding wayle to the sharedModules list inside of
                            # your home manager module is the important part.
                            inputs.wayle.homeManagerModules.default
                        ];
                        # ...
                    };
                }
                # ...
            ];
        };

        # ...
    };

}
```

Finally, you can enable wayle in your `home.nix` file using:

```nix
# home.nix
{
    config,
    lib,
    pkgs,
    inputs, # Only needed to install the wayle package manually.
    ...
}: {
    # ...

    services.wayle = {
        enable = true;
        settings = {
            # Put your wayle configuration here.
        };

        # Uncomment this if you don't want wayle to start automatically:
        # systemd.enable = false;
    };

    # -- Optional --
    # If you would like to access the `wayle` and `wayle-shell` commands from
    # your command line, then remember to add wayle as a package to your home
    # environment using the following.
    #
    # This is required if you set `services.wayle.systemd.enable = false`
    # above. Otherwise you will have no way to start wayle.
    home.packages = with pkgs; [
        # ...
        inputs.wayle.packages.${stdenv.hostPlatform.system}.default
        # ...
    ];

    # ...
}
```

#### Notes on using NixOS

##### Icons

This nix package will automatically install the icons for you so you don't need
to worry about running `wayle icons setup`.

##### Live Config Reloading

NixOS will generate the wayle `config.toml` file for you using the config you
wrote for wayle in your `home.nix` file. Unfortunately, this misses out on
wayle's live config reloading feature. To work around this and configure wayle
without rebuilding your home-manager config everytime and to allow utilizing the
`wayle config` the following steps are recommended:

1. Move the nix-made symlink config file to a new path and make a clone of it to
   be used as a starting point with:
   ```bash
   mv ~/.config/wayle/config.toml{,.bak}
   cp ~/.config/wayle/config.toml{.bak,}
   ```

2. Modify the config file at `~/.config/wayle/config.toml` either manually or by
   using the `wayle config <cmd>` commands until you are happy with your
   configuration.

3. Translate your new config from the TOML syntax of the
   `~/.config/wayle/config.toml` file into nix syntax in your `home.nix` file at
   `services.wayle.settings = { # settings go here }`.

4. Move the original config symlink back (this will overwrite whatever changes
   you may have made and return you to the config defined by your nix config).
   ```bash
   mv ~/.config/wayle/config.toml{.bak,}
   ```

5. Rebuild your home-manager config and the new settings will be applied.

This method allows you to configure wayle efficiently and benefit from its live
config reloading and command line features, while still declaratively writing
your config using nix.

**CAUTION:** Home manager will automatically move any existing config to a
`config.toml.hm-bak`, or similar, backup file name. This is expected, however,
wayle will automatically create a _new_ `config.toml` file if you moved the
symlink make by home-manager and didn't move it back. If this occurs,
home-manager will try to move the new `config.toml` (non-symlink) file to
`config.toml.hm-bak` (or similar) _again_ and fail because that back up file
already exists. If you are rebuilding with nix, and it is failing, check if your
`~/.config/wayle` directory contains these non-symlinked files and remove or
re-backup them to a different file name. Home assistant seems to give very
little to no feedback when this error occurs.

</details>

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

## Custom Modules

Custom modules run shell commands and display the output in the bar. Define one
in your config and add it to your layout with the `custom-` prefix:

```toml
[[bar.layout]]
monitor = "*"
right = ["custom-gpu-temp", "clock"]

[[modules.custom]]
id = "gpu-temp"
command = "nvidia-smi --query-gpu=temperature.gpu --format=csv,noheader,nounits"
interval-ms = 5000
format = "{{ output }}°C"
icon-name = "ld-thermometer-symbolic"
```

The `command` runs via `sh -c`. Plain text output is available as `{{ output }}`
in `format`. If the output starts with `{` or `[`, it's parsed as JSON and
fields are available directly: `{{ temperature }}`, `{{ nested.value }}`, etc.

### Execution Modes

**Poll** (default) runs the command every `interval-ms` milliseconds:

```toml
# default, can be omitted
mode = "poll"
# every 5 seconds
interval-ms = 5000
```

**Watch** spawns the command once and updates the display on each line of
stdout. Good for commands that stream events like `pactl subscribe` or
`inotifywait`:

```toml
[[modules.custom]]
id = "volume"
mode = "watch"
command = '''
pactl subscribe | while read -r line; do
  if [[ "$line" == *"sink"* ]]; then
    vol=$(pactl get-sink-volume @DEFAULT_SINK@ | grep -oP '\d+(?=%)' | head -1)
    echo "{\"percentage\": $vol}"
  fi
done
'''
format = "{{ percentage }}%"
restart-policy = "on-failure"
```

If a watch process exits, `restart-policy` controls what happens:

- `never` (default) - stay dead
- `on-exit` - restart after any exit
- `on-failure` - restart only on non-zero exit codes

The restart delay starts at `restart-interval-ms` (default 1000ms) and doubles
on each rapid failure, capping at 30 seconds.

### Dynamic Icons

If your command outputs JSON with a `percentage` field (0-100), you can map it
to an array of icons. The array is divided evenly across the range:

```toml
[[modules.custom]]
id = "battery"
command = '''
cap=$(cat /sys/class/power_supply/BAT0/capacity)
echo "{\"percentage\": $cap}"
'''
interval-ms = 30000
format = "{{ percentage }}%"
icon-names = [
  "ld-battery-warning-symbolic",
  "ld-battery-low-symbolic",
  "ld-battery-medium-symbolic",
  "ld-battery-full-symbolic"
]
```

4 icons means: 0-24% picks the first, 25-49% the second, 50-74% the third,
75-100% the fourth.

For state-based icons, output an `alt` field and use `icon-map`:

```toml
icon-map = { muted = "ld-volume-off-symbolic", default = "ld-volume-2-symbolic" }
```

If both `alt` and `percentage` are present, `icon-map` wins. The full priority
is: `icon-map[alt]` > `icon-names[percentage]` > `icon-map["default"]` >
`icon-name`.

### Click Actions

Each interaction type has its own command:

```toml
left-click = "pavucontrol"
scroll-up = "pactl set-sink-volume @DEFAULT_SINK@ +5%"
scroll-down = "pactl set-sink-volume @DEFAULT_SINK@ -5%"
```

By default, the display won't update until the next poll. To refresh immediately
after an action, add `on-action` - its output updates the display right away:

```toml
on-action = '''
vol=$(pactl get-sink-volume @DEFAULT_SINK@ | grep -oP '\d+(?=%)' | head -1)
echo "{\"percentage\": $vol}"
'''
```

Scroll events are debounced (50ms) so rapid scrolling doesn't fire dozens of
commands. Set `interval-ms = 0` if you only want updates from `on-action` (no
polling at all).

### JSON Reserved Fields

When outputting JSON, these fields have special meaning:

| Field        | Type         | Effect                                     |
| ------------ | ------------ | ------------------------------------------ |
| `text`       | string       | Replaces the `format` result for the label |
| `tooltip`    | string       | Replaces the `tooltip-format` result       |
| `percentage` | number       | 0-100, selects from `icon-names`           |
| `alt`        | string       | Selects from `icon-map`                    |
| `class`      | string/array | Adds CSS classes to the module             |

All other fields are available in `format` and `tooltip-format` templates.

### Full Reference

<details>
<summary>All fields for <code>[[modules.custom]]</code></summary>

#### Core

| Field                 | Type                                     | Default   | Description                                                    |
| --------------------- | ---------------------------------------- | --------- | -------------------------------------------------------------- |
| `id`                  | string                                   | required  | Unique ID, referenced in layout as `custom-<id>`               |
| `command`             | string                                   | none      | Shell command (`sh -c`). JSON auto-detected                    |
| `mode`                | `"poll"` / `"watch"`                     | `"poll"`  | Poll runs on interval, watch streams stdout                    |
| `interval-ms`         | number                                   | `5000`    | Poll interval. `0` = manual only. Ignored in watch mode        |
| `restart-policy`      | `"never"` / `"on-exit"` / `"on-failure"` | `"never"` | Watch mode only                                                |
| `restart-interval-ms` | number                                   | `1000`    | Watch mode restart delay (doubles on rapid failures, caps 30s) |

#### Display

| Field            | Type   | Default          | Description                                               |
| ---------------- | ------ | ---------------- | --------------------------------------------------------- |
| `format`         | string | `"{{ output }}"` | Template for the label. Use `{{ field }}` for JSON fields |
| `tooltip-format` | string | none             | Template for hover tooltip                                |
| `hide-if-empty`  | bool   | `false`          | Hide when output is empty, `"0"`, or `"false"`            |
| `class-format`   | string | none             | Template for dynamic CSS classes (space-separated)        |

#### Icons

| Field        | Type     | Default | Description                                            |
| ------------ | -------- | ------- | ------------------------------------------------------ |
| `icon-name`  | string   | `""`    | Static fallback icon                                   |
| `icon-names` | string[] | none    | Icons indexed by JSON `percentage` (0-100)             |
| `icon-map`   | table    | none    | Icons keyed by JSON `alt`. `"default"` key as fallback |

#### Styling

| Field              | Type   | Default       | Description                             |
| ------------------ | ------ | ------------- | --------------------------------------- |
| `icon-show`        | bool   | `true`        | Show the icon                           |
| `icon-color`       | color  | `"auto"`      | Icon foreground color                   |
| `icon-bg-color`    | color  | `"auto"`      | Icon container background               |
| `label-show`       | bool   | `true`        | Show the text label                     |
| `label-color`      | color  | `"auto"`      | Label text color                        |
| `label-max-length` | number | `0`           | Truncate after N chars (`0` = no limit) |
| `button-bg-color`  | color  | theme default | Button background                       |
| `border-show`      | bool   | `false`       | Show border                             |
| `border-color`     | color  | `"auto"`      | Border color                            |

#### Actions

| Field          | Type   | Default | Description                                   |
| -------------- | ------ | ------- | --------------------------------------------- |
| `left-click`   | string | `""`    | Command on left click                         |
| `right-click`  | string | `""`    | Command on right click                        |
| `middle-click` | string | `""`    | Command on middle click                       |
| `scroll-up`    | string | `""`    | Command on scroll up (50ms debounce)          |
| `scroll-down`  | string | `""`    | Command on scroll down (50ms debounce)        |
| `on-action`    | string | none    | Runs after any action, output updates display |

Color values: `"auto"`, hex (`"#ff0000"`), or theme token (`"red"`, `"primary"`,
etc.).

</details>

## Credits

Big thanks to [@M70v](https://www.instagram.com/m70v.art/) for his Wayle logo
contribution! Check out his work at <https://www.instagram.com/m70v.art/>.

## License

MIT
