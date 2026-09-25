use std::io::{self, Write};

use anyhow::{anyhow, Result};

use crate::{actions::handle::handle_action, config::Config};

mod actions;
mod config;
mod gui;
mod os;

fn main() -> Result<()> {
    let config = Config::load()?;

    print!("[macro]> ");
    io::stdout().flush()?;

    let mut ip = String::new();
    io::stdin().read_line(&mut ip)?;

    let keyword = ip.trim();
    if keyword.is_empty() {
        return Err(anyhow!("No macro entered"));
    }

    let entry = config.get_action(keyword)?;
    handle_action(entry)?;

    Ok(())
}
