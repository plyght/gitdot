use gitdot_metrics::GitdotMetricsServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = GitdotMetricsServer::new().await?;
    server.start().await
}
