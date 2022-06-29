use cloudrave::Sound;
use futures::StreamExt;
use rodio::OutputStream;
use rodio::Source;
use std::time::Duration;

// TODO restart if panic?

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let sound = Sound::load("assets/kick.ogg")?;
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default()?;

    // let transport = web3::transports::Http::new("https://rpc.testnet.cloudwalk.io")?;
    let transport = web3::transports::WebSocket::new("wss://mainnet.cloudwalk.io/ws").await?;
    let web3 = web3::Web3::new(transport);

    let brlc_contract = "0xA9a55a81a4C085EC0C31585Aed4cFB09D78dfD53";

    // let filter = web3::types::FilterBuilder::default()
    //     .from_block(web3::types::BlockNumber::Latest)
    //     .build();

    // let mut logs = web3.eth_subscribe().subscribe_logs(filter).await?;
    let mut logs = web3.eth_subscribe().subscribe_new_heads().await?;

    while let Some(result) = logs.next().await {
        let log = result?;
        println!("{:?}", log);

        stream_handle.play_raw(sound.decoder().convert_samples());
        stream_handle.play_raw(
            sound
                .decoder()
                .delay(Duration::from_millis(500))
                .convert_samples(),
        );
    }

    Ok(())
}
