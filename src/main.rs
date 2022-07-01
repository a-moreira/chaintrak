#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stream = cloudrave::event_streamer::start().await?;
    cloudrave::clouddrums::play(stream).await
}
