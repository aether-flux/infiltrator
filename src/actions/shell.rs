use anyhow::{Context, Result};

use crate::{actions::clipboard::clipboard_action, os::shell::create_command};

pub fn shell_action(cmd: &str) -> Result<()> {
    create_command(cmd)
        .status()
        .with_context(|| format!("Failed to spawn command: {}", cmd))?;

    Ok(())
}

pub fn shell_output_action(cmd: &str) -> Result<()> {
    let output = create_command(cmd)
        .output()
        .with_context(|| format!("Failed to spawn command: {}", cmd))?;

    if !output.status.success() {
        let stderr_msg = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!(
            "Command failed with exit code {:?}: {}",
            output.status.code(),
            stderr_msg.trim()
        );
    }

    let stdout_str =
        String::from_utf8(output.stdout).context("Command output was not a valid UTF-8")?;
    let trimmed = stdout_str.trim();

    clipboard_action(trimmed)?;

    Ok(())
}
