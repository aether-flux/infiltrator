use std::{
    process::{Command, Stdio},
    thread,
    time::Duration,
};

use anyhow::Result;
use enigo::{Enigo, Keyboard, Settings};

use crate::os::is_wayland;

pub fn type_text(text: &str) -> Result<()> {
    // Delay: allow 'infiltrator' process/window to exit
    thread::sleep(Duration::from_secs(3));

    if is_wayland() {
        // Wayland
        println!("wayland detected");
        let mut wtype = Command::new("wtype")
            .arg("--")
            .arg(text)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        wtype.wait()?;
    } else {
        // X11
        let mut enigo = Enigo::new(&Settings::default())
            .map_err(|e| anyhow::anyhow!("Failed to initiate enigo on X11: {}", e))?;
        enigo
            .text(text)
            .map_err(|e| anyhow::anyhow!("Enigo text synthesis failed on X11: {}", e))?;
    }

    Ok(())
}
