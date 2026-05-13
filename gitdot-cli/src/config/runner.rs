use anyhow::Context;
use figment::{
    Figment,
    providers::{Format, Serialized, Toml},
};
use serde::{Deserialize, Serialize};

use crate::util::url::{API_SERVER_URL, S2_SERVER_URL, WEB_URL};

const SYSTEM_USER: &str = "gitdot";
const RUNNER_CONFIG_PATH: &str = "/etc/gitdot/runner.toml";
const DEFAULT_NUM_EXECUTORS: i8 = 4;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunnerConfig {
    pub gitdot_web_url: String,
    pub gitdot_server_url: String,
    pub s2_server_url: String,
    pub runner_token: Option<String>,
    pub num_executors: i8,
}

impl Default for RunnerConfig {
    fn default() -> Self {
        Self {
            gitdot_server_url: API_SERVER_URL.to_string(),
            gitdot_web_url: WEB_URL.to_string(),
            s2_server_url: S2_SERVER_URL.to_string(),
            runner_token: None,
            num_executors: DEFAULT_NUM_EXECUTORS,
        }
    }
}

impl RunnerConfig {
    pub fn load() -> anyhow::Result<Self> {
        Figment::new()
            .merge(Serialized::defaults(Self::default()))
            .merge(Toml::file(RUNNER_CONFIG_PATH))
            .extract()
            .with_context(|| format!("Failed to load runner config from {RUNNER_CONFIG_PATH}"))
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let contents = toml::to_string_pretty(self).context("Failed to serialize runner config")?;

        let tmp = std::env::temp_dir().join("gitdot-runner.toml");
        std::fs::write(&tmp, &contents).context("Failed to write runner config to temp file")?;
        let tmp_str = tmp.to_str().context("Temp path is not valid UTF-8")?;

        crate::util::command::run_command("sudo", &["mkdir", "-p", "/etc/gitdot"])
            .context("Failed to create /etc/gitdot directory")?;
        crate::util::command::run_command("sudo", &["cp", tmp_str, RUNNER_CONFIG_PATH])
            .context("Failed to install runner config to /etc/gitdot/runner.toml")?;

        Ok(())
    }
}
