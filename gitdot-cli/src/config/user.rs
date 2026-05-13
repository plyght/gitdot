use std::path::PathBuf;

use anyhow::Context;
use figment::{
    Figment,
    providers::{Env, Format, Serialized, Toml},
};
use serde::{Deserialize, Serialize};

use crate::util::url::{API_SERVER_URL, AUTH_SERVER_URL, WEB_URL};

const CONFIG_DIR_NAME: &str = "gitdot";
const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub gitdot_web_url: String,
    pub gitdot_api_server_url: String,
    pub gitdot_auth_server_url: String,
    pub user_name: String,
    pub user_email: String,
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            gitdot_web_url: WEB_URL.to_string(),
            gitdot_api_server_url: API_SERVER_URL.to_string(),
            gitdot_auth_server_url: AUTH_SERVER_URL.to_string(),
            user_name: String::new(),
            user_email: String::new(),
        }
    }
}

impl UserConfig {
    pub fn load() -> anyhow::Result<Self> {
        let config_path = Self::get_config_path()?;

        // you can override config values via environment variable
        Figment::new()
            .merge(Serialized::defaults(Self::default()))
            .merge(Toml::file(&config_path))
            .merge(Env::raw())
            .extract()
            .with_context(|| format!("Failed to load config from {}", config_path.display()))
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = Self::get_config_path()?;
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create config directory: {}", parent.display())
            })?;
        }

        let contents = toml::to_string_pretty(self).context("Failed to serialize config")?;
        std::fs::write(&config_path, contents)
            .with_context(|| format!("Failed to write config file: {}", config_path.display()))?;

        Ok(())
    }

    fn get_config_path() -> anyhow::Result<PathBuf> {
        let base_dir = dirs_next::config_dir().context("Could not determine config directory")?;
        let config_dir = base_dir.join(CONFIG_DIR_NAME);
        Ok(config_dir.join(CONFIG_FILE_NAME))
    }
}
