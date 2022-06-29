use futures::StreamExt;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
use rodio;
use std::io;
use std::convert::AsRef;
use std::sync::Arc;
use std::io::Read;
use std::time::Duration;

pub struct Sound (Arc<Vec<u8>>);

impl AsRef<[u8]> for Sound {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Sound {
    pub fn load(filename: &str) -> io::Result<Sound> {
        let mut buf = Vec::new();
        let mut file = File::open(filename)?;
        file.read_to_end(&mut buf)?;
        Ok(Sound(Arc::new(buf)))
    }
    pub fn cursor(self: &Self) -> io::Cursor<Sound> {
        io::Cursor::new(Sound(self.0.clone()))
    }
    pub fn decoder(self: &Self) -> rodio::Decoder<io::Cursor<Sound>> {
        rodio::Decoder::new(self.cursor()).unwrap()
    }
}

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

    let filter = web3::types::FilterBuilder::default()
        .from_block(web3::types::BlockNumber::Latest)
        .build();

    // let mut logs = web3.eth_subscribe().subscribe_logs(filter).await?;
    let mut logs = web3.eth_subscribe().subscribe_new_heads().await?;

    while let Some(result) = logs.next().await {
        let log = result?;
        println!("{:?}", log);

        stream_handle.play_raw(sound.decoder().convert_samples());
        stream_handle.play_raw(sound.decoder().delay(Duration::from_millis(500)).convert_samples());
    }

    Ok(())
}
