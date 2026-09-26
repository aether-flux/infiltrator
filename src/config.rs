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
    Text { value: String },
    Shell { cmd: String },
    ShellOutput { cmd: String },
    Clipboard { value: String },
    Open { url: String },
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Theme {
    pub primary: Option<[u8; 3]>,
    pub secondary: Option<[u8; 3]>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub theme: Theme,
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
"browser" = { type = "open", url = "https://github.com" }
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
