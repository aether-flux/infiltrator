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
        thread::sleep(Duration::from_millis(30));

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

    // // Delay: allow 'infiltrator' process/window to exit
    // thread::sleep(Duration::from_millis(30));
    //
    // if is_wayland() {
    //     // Wayland
    //     let mut wtype = Command::new("wtype")
    //         .arg("--")
    //         .arg(text)
    //         .stdout(Stdio::null())
    //         .stderr(Stdio::null())
    //         .spawn()?;
    //
    //     wtype.wait()?;
    // } else {
    //     // X11
    //     let mut enigo = Enigo::new(&Settings::default())
    //         .map_err(|e| anyhow::anyhow!("Failed to initiate enigo on X11: {}", e))?;
    //     enigo
    //         .text(text)
    //         .map_err(|e| anyhow::anyhow!("Enigo text synthesis failed on X11: {}", e))?;
    // }

    Ok(())
}
