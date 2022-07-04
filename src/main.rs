use chaintrak::args::Args;
use clap::Parser;
use log::LevelFilter;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    SimpleLogger::new().with_level(LevelFilter::Info).init()?;
    let args = Args::parse();

    let stream = chaintrak::event_streamer::start().await?;
    chaintrak::engine::play(stream, args.vibe).await
}
