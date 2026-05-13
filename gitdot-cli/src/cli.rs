use crate::{command::Args, config::UserConfig};

pub async fn run(args: &Args) -> anyhow::Result<()> {
    let config = UserConfig::load().await?;
    match args {
        Args::Login(login_args) => login_args.execute(config).await,
        Args::Status(status_args) => status_args.execute(config).await,
        // TODO: re-enable as these features ship.
        // Args::Save(save_args) => save_args.execute().await,
        // Args::Ci(ci_args) => ci_args.command.execute().await,
        // Args::Review(review_args) => review_args.command.execute(config).await,
        // Args::Runner(runner_args) => {
        //     let config = crate::config::RunnerConfig::load()?;
        //     runner_args.command.execute(config).await
        // }
    }
}
