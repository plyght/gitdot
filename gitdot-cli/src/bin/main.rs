use clap::Parser;

use gitdot_cli::{Args, run, setup};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup().await?;

    let args = Args::parse();
    run(&args).await
}
