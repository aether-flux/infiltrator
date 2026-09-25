use anyhow::Result;
use copypasta::{ClipboardContext, ClipboardProvider};

pub fn clipboard_action(value: &String) -> Result<()> {
    let mut ctx = ClipboardContext::new()
        .map_err(|e| anyhow::anyhow!("Failed to initialize clipboard provider: {}", e))?;
    ctx.set_contents(value.to_owned())
        .map_err(|e| anyhow::anyhow!("Failed to set clipboard contents: {}", e))?;

    Ok(())
}
