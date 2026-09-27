use std::{
    process::{Command, Stdio},
    thread,
    time::Duration,
};

use anyhow::Result;
use enigo::{Enigo, Keyboard, Settings};

use crate::os::is_wayland;

/// Runs the binary with the hidden helper flag `--type-text`
pub fn type_text(text: &str) -> Result<()> {
    let exe = std::env::current_exe()?;
    Command::new(exe)
        .arg("--type-text")
        .arg(text)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(())
}

/// Called only in the `--type-text` mode. Waits and runs command to simulate required keystrokes.
pub fn type_text_blocking(text: &str) -> Result<()> {
    thread::sleep(Duration::from_millis(300));

    if is_wayland() {
        let wayland_display =
            std::env::var("WAYLAND_DISPLAY").unwrap_or_else(|_| "wayland-0".to_string());
        let xdg_runtime = std::env::var("XDG_RUNTIME_DIR").unwrap_or_default();

        Command::new("wtype")
            .arg("--")
            .arg(text)
            .env("WAYLAND_DISPLAY", wayland_display)
            .env("XDG_RUNTIME_DIR", xdg_runtime)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output()?;
    } else {
        let mut enigo = Enigo::new(&Settings::default())?;
        enigo.text(text)?;
    }

    Ok(())
}
