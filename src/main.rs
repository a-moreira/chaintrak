use chaintrak::args::Args;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let stream = chaintrak::event_streamer::start().await?;
    chaintrak::engine::play(stream, args.vibe).await
}
