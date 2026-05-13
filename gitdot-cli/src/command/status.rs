use clap::Args;

use crate::config::UserConfig;

#[derive(Args, Debug)]
pub struct StatusArgs;

impl StatusArgs {
    pub async fn execute(&self, config: UserConfig) -> anyhow::Result<()> {
        if config.user_name.is_empty() {
            println!("Not logged in");
        } else {
            println!("Logged in as {}", config.user_name);
        }
        Ok(())
    }
}
