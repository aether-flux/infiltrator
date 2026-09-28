# Infiltrator
tags

Infiltrator is an aesthetic, keyboard-driven macro overlay utility designed to streamline repetitive workflows.
media/screenshot

# Features
Infiltrator is able to support the following features:
- Minimal UI with ambient glows and waves, powered by [`egui`](https://crates.io/crates/egui) and [`eframe`](https://crates.io/crates/eframe).
- Instant text expansion into focused window.
- Shell command execution, output-ed into the clipboard.
- URL launching in default browser.
- Configurable theme colors and macro actions via a unified config file at `~/.config/infiltrator/config.toml`.
- Automatic default config creation on first run.

# Pre-requisites (Currently Supported)
- **OS**: Linux
- **Protocol**: Wayland (/ X11)
- **Runtime Dependencies**: `wtype`, `wl-clipboard`

> [!WARNING]
> Infiltrator can still run in X11, however text expansion and shell output clipboard actions may not work properly.

# Installation
There are various ways to install Infiltrator on your system.

### Arch Linux
```sh
yay -S infiltrator-bin
```

### Install script
```sh
curl -sSL https://raw.githubusercontent.com/aether-flux/infiltrator/main/install.sh | bash
```

### Manual Build
```sh
# Clone the repository
git clone https://github.com/aether-flux/infiltrator
cd infiltrator

# Build binary
cargo build --release

# Copy binary into system or local PATH ($HOME/.local/bin or /usr/bin)
cp target/release/infiltrator $HOME/.local/bin

# Copy desktop file and icon (Optional)
cp assets/infiltrator.desktop $HOME/.local/share/applications
cp assets/infiltrator.png $HOME/.local/share/icons/hicolor/48x48/apps/

# Installation complete!
# Add keybindings depending on your desktop environment or window manager.
```

# Configuration
As previously mentioned, infiltrator automatically creates a default config file if not already present. The default config is as follows:
```toml
[macros]
"date" = { type = "shell_output", cmd = "date '+%d/%m/%Y'" }
"time" = { type = "shell_output", cmd = "date '+%H:%M'" }
"email" = { type = "text", value = "user@example.com" }
"infiltrator" = { type = "open", url = "https://github.com/aether-flux/infiltrator" }
```

### Macro Actions
There are three types of actions currently supported:
- `shell_output`: Executes a command and copies the output to your clipboard.
- `text`: Types out the given text value in the focused text window.
- `open`: Opens the specified URL in your default browser.

So when `date` is typed, it will copy today's date into your clipboard. On `email`, it will type out your saved email address into the focused text window. `infiltrator` would just open this GitHub page in your default browser.

### Theme
You can also customize the colors used in infiltrator. The config supports a `[theme]` section that lets you set a primary and secondary color, each being a array of 3 numbers as [r, g, b].
This is the default value of the colors, which are used if no colors are specified in the config:
```toml
[theme]
primary = [255, 15, 123]
secondary = [248, 155, 41]
```

You can use your color of choice by playing with these values.

# Keybinding Guide
Infiltrator doesn't come with its own keybindings. Instead, it lets the user decide which keys to bind and how to bind them to run it.
A few common desktop or window managers:
- **Hyprland (Lua)**: `hl.bind(\"SUPER + Space\", hl.dsp.exec_cmd(\"infiltrator\"))`
- **Sway/i3**: `bindsym \$mod+space exec infiltrator`
- **sxhkd (& bspwm)**:
  ```sxhkdrc
  super + space
    infiltrator
  ```
- **KDE, Gnome, etc**: Add custom shortcut for infiltrator in the system settings.

# Credits & Acknowledgements
- `eframe` / `egui`: The core of the GUI.
- [`open-rs`](https://github.com/Byron/open-rs): Borrowed its code and approach of trying out multiple commands to open URLs in browsers.
- [Google Sans Flex](https://fonts.google.com/specimen/Google+Sans+Flex?preview.script=Latn): The font used in the GUI.

# License
MIT
