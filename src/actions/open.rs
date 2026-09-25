use std::process::Stdio;

use anyhow::{Context, Result};

use crate::os::open::open_commands;

pub fn open_action(url: &str) -> Result<()> {
    let mut last_err = None;
    for mut cmd in open_commands(url) {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());

        match cmd.spawn() {
            Ok(_) => return Ok(()),
            Err(e) => last_err = Some(e),
        }
    }

    Err(last_err
        .context("No launcher worked, at least one error")?
        .into())
}
