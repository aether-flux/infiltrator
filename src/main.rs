use anyhow::Result;

use crate::{config::Config, gui::run_overlay};

mod actions;
mod config;
mod gui;
mod os;

fn main() -> Result<()> {
    let config = Config::load()?;

    run_overlay(config)?;
    Ok(())
}
