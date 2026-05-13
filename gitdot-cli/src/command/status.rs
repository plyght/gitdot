use clap::Args;
use owo_colors::OwoColorize;

use crate::config::UserConfig;

#[derive(Args, Debug)]
pub struct StatusArgs;

impl StatusArgs {
    pub async fn execute(&self, config: UserConfig) -> anyhow::Result<()> {
        if config.user_name.is_empty() {
            println!("{}", "Not logged in".yellow());
        } else {
            println!(
                "{} {}",
                "Logged in as".green(),
                config.user_name.cyan().bold()
            );
        }
        Ok(())
    }
}
