use std::{
    process::{Command, Stdio},
    thread,
    time::Duration,
};

use anyhow::Result;
use enigo::{Enigo, Keyboard, Settings};

use crate::os::is_wayland;

pub fn type_text(text: &str) -> Result<()> {
    let text = text.to_string();
    let wayland = is_wayland();

    thread::spawn(move || {
        // Delay: allow 'infiltrator' process/window to exit
        thread::sleep(Duration::from_millis(50));
        // thread::sleep(Duration::from_secs(3));

        if wayland {
            // Wayland
            let _ = Command::new("wtype")
                .arg("--")
                .arg(text)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .and_then(|mut child| child.wait());
        } else if let Ok(mut enigo) = Enigo::new(&Settings::default()) {
            // X11
            let _ = enigo.text(&text);
        }
    });

    Ok(())
}
