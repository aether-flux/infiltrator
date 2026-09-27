use anyhow::{Context, Result};

use crate::os::keyboard::type_text;

/// Type the specified text into current focused window
pub fn text_action(text: &str) -> Result<()> {
    type_text(text).context("Failed to inject text into focused window")?;
    Ok(())
}
