#![allow(unreachable_patterns)]

use anyhow::Result;

use crate::{
    actions::{
        clipboard::clipboard_action,
        keyboard::text_action,
        open::open_action,
        shell::{shell_action, shell_output_action},
    },
    config::MacroAction,
};

pub fn handle_action(action: &MacroAction) -> Result<()> {
    match action {
        MacroAction::Text { value } => text_action(value)?,
        MacroAction::Shell { cmd } => shell_action(cmd)?,
        MacroAction::Clipboard { value } => clipboard_action(value)?,
        MacroAction::Open { url } => open_action(url)?,
        MacroAction::ShellOutput { cmd } => shell_output_action(cmd)?,

        _ => {}
    }

    Ok(())
}
