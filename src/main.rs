use futures::StreamExt;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default()?;

    // let transport = web3::transports::Http::new("https://rpc.testnet.cloudwalk.io")?;
    let transport = web3::transports::WebSocket::new("wss://mainnet.cloudwalk.io/ws").await?;
    let web3 = web3::Web3::new(transport);

    let brlc_contract = "0xA9a55a81a4C085EC0C31585Aed4cFB09D78dfD53";

    let filter = web3::types::FilterBuilder::default()
        .from_block(web3::types::BlockNumber::Latest)
        .build();

    // let mut logs = web3.eth_subscribe().subscribe_logs(filter).await?;
    let mut logs = web3.eth_subscribe().subscribe_new_heads().await?;

    while let Some(result) = logs.next().await {
        let log = result?;
        println!("{:?}", log);
        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open("assets/kick.ogg")?);
        // Decode that sound file into a source
        let source = Decoder::new(file)?;
        // Play the sound directly on the device
        stream_handle.play_raw(source.convert_samples());
    }

    Ok(())
}
