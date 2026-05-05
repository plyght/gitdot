use gitdot_consumer::GitdotConsumer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let consumer = GitdotConsumer::new().await?;
    consumer.run().await
}
