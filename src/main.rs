use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    vibe: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let vibe = args.vibe;

    let stream = chaintrak::event_streamer::start().await?;
    chaintrak::engine::play(stream, vibe).await
}
