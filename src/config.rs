use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Result, anyhow};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MacroAction {
    Text { value: String },
    Shell { cmd: String },
    ShellOutput { cmd: String },
    Clipboard { value: String },
    Open { url: String },
}

#[derive(Debug, Deserialize)]
pub struct Config {
    macros: HashMap<String, MacroAction>,
}

impl Config {
    /// Load config path
    pub fn load() -> Result<Self> {
        let path = Self::get_config_path()?;

        if !path.exists() {
            Self::create_default_config(&path)?;
        }

        let content = fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&content)?;

        Ok(config)
    }

    /// Get path to config file
    pub fn get_config_path() -> Result<PathBuf> {
        let path = PathBuf::from("./config.toml");
        Ok(path)
    }

    /// Create default config if file doesn't exist
    pub fn create_default_config(path: &Path) -> Result<()> {
        let default_toml = r#"
[macros]
":ip:" = { type = "shell", cmd = "curl -s ifconfig.me" }
"email" = { type = "text", value = "user@example.com" }
":clip:" = { type = "clipboard", value = "Infiltrator Macro Active!" }
"browser" = { type = "open", url = "https://github.com" }
"#;

        fs::write(path, &default_toml.trim())?;
        Ok(())
    }

    /// Get entry action of given macro
    pub fn get_action(&self, macro_val: &str) -> Result<&MacroAction> {
        if let Some(m) = self.macros.get(macro_val) {
            return Ok(m);
        } else {
            return Err(anyhow!("Input macro has no action associated with it"));
        }
    }
}
