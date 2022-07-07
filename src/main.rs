use chaintrak::args::Args;
use clap::Parser;
use log::LevelFilter;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    SimpleLogger::new().with_level(LevelFilter::Info).init()?;
    let args = Args::parse();

    let events = chaintrak::events::streamer::start().await?;
    args.vibe.play(events).await
}
