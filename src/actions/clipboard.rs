use std::process::Stdio;

use anyhow::{Context, Result};

use crate::os::clipboard::clipboard_commands;

pub fn clipboard_action(value: &str) -> Result<()> {
    let mut last_err = None;
    for mut cmd in clipboard_commands(value) {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());

        match cmd.spawn() {
            Ok(_) => return Ok(()),
            Err(e) => last_err = Some(e),
        }
    }

    Err(last_err
        .context("No clipboard manager worked, at least one error")?
        .into())
}
