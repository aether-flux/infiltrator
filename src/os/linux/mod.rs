//! This module contains everything that works in `linux` operating system.

pub mod clipboard;
pub mod keyboard;
pub mod open;
pub mod shell;

/// Returns `true` if the current session is **Wayland**
pub fn is_wayland() -> bool {
    std::env::var("XDG_SESSION_TYPE")
        .map(|s| s.eq_ignore_ascii_case("wayland"))
        .unwrap_or_else(|_| std::env::var("WAYLAND_DISPLAY").is_ok())
}
