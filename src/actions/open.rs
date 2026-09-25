use std::process::Stdio;

use anyhow::{Context, Result};

use crate::os::open;

pub fn open_action(url: &String) -> Result<()> {
    let mut last_err = None;
    for mut cmd in open::commands(url) {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());

        // `spawn` launches process asynchronously
        match cmd.spawn() {
            Ok(_) => return Ok(()),
            Err(e) => last_err = Some(e),
        }
    }

    Err(last_err
        .context("No launcher worked, at least one error")?
        .into())
}
