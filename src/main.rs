//! # Infiltrator
//!
//! Infiltrator is an aesthetic, keyboard-driven macro overlay utility designed to streamline
//! repetitive workflows. It supports triggering system actions, pasting dynamic shell command
//! output, expanding static text blocks, and opening URLs in browsers.
//!
//! The application uses [`egui`](https://docs.rs/egui) to render a minimalist interface featuring
//! customizable animated dynamic waves and ambient glows. It uses *Google Sans Flex* as the
//! typography.
//!
//! ## Configuration
//! On first launch, `infiltrator` automatically creates a default config file if one doesn't exist at `~/.config/infiltrator/config.toml`.
//!
//! ### Default macros (config)
//! ```toml
//! [macros]
//! "date" = { type = "shell_output", cmd = "date '+%d/%m/%Y'" }
//! "time" = { type = "shell_output", cmd = "date '+%H:%M'" }
//! "email" = { type = "text", value = "user@example.com" }
//! "infiltrator" = { type = "open", url = "https://github.com/aether-flux/infiltrator" }
//! ```
//!
//! ### Action types
//! The `type` field in `[macros]` determines how the key trigger is executed:
//! - `shell_output`: Executes a shell command and copies the stdout to clipboard.
//! - `text`: Simulates typing key events to expand the specified string into the active window.
//! - `open`: Opens the given URL in your system's default browser.
//!
//! ### Custom themes
//! Colors can be configured under the `[theme]` section using RGB array tuples. If omitted,
//! `infiltrator` falls back to its default ambient palette:
//!
//! ```toml
//! [theme]
//! primary = [255, 15, 123]
//! secondary = [248, 155, 41]
//! ```

use anyhow::Result;

use crate::{config::Config, gui::run_overlay};

mod actions;
mod config;
mod gui;
mod os;

fn main() -> Result<()> {
    // Helper mode: '--type-text' argument runs the command to simulate keystrokes in a separate process
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--type-text") {
        let text = args.get(2).cloned().unwrap_or_default();
        return os::keyboard::type_text_blocking(&text);
    }

    let config = Config::load()?;
    run_overlay(config)?;
    Ok(())
}
