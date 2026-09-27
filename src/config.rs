//! This module handles the configuration options of `infiltrator`.
//! In Linux, the config file is stored at `~/.config/infiltrator/config.toml`/

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Result, anyhow};
use directories::ProjectDirs;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MacroAction {
    /// Type given text value in focused window
    Text { value: String },
    /// Run a command and copy the output to clipboard
    ShellOutput { cmd: String },
    /// Open a URL in the browser
    Open { url: String },
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Theme {
    /// Primary color (r, g, b)
    pub primary: Option<[u8; 3]>,
    /// Secondary color (r, g, b)
    pub secondary: Option<[u8; 3]>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    /// Set theme
    #[serde(default)]
    pub theme: Theme,
    /// Set macro keywords and actions
    macros: HashMap<String, MacroAction>,
}

impl Config {
    /// Load config from file
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
        if let Some(proj_dirs) = ProjectDirs::from("", "", "infiltrator") {
            let config_dir = proj_dirs.config_dir();
            fs::create_dir_all(config_dir)?;
            Ok(config_dir.join("config.toml"))
        } else {
            Err(anyhow::anyhow!("Failed to resolve user config directory"))
        }
    }

    /// Create default config if file doesn't exist
    pub fn create_default_config(path: &Path) -> Result<()> {
        let default_toml = r#"
[macros]
"date" = { type = "shell_output", cmd = "date '+%d/%m/%Y'" }
"time" = { type = "shell_output", cmd = "date '+%H:%M'" }
"email" = { type = "text", value = "user@example.com" }
"infiltrator" = { type = "open", url = "https://github.com/aether-flux/infiltrator" }
"#;

        fs::write(path, default_toml.trim())?;
        Ok(())
    }

    /// Get entry action of given macro
    pub fn get_action(&self, macro_val: &str) -> Result<&MacroAction> {
        if let Some(m) = self.macros.get(macro_val) {
            Ok(m)
        } else {
            Err(anyhow!("Input macro has no action associated with it"))
        }
    }
}
