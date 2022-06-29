use cloudrave::{sample::Samples, Event};
use tokio_stream::{StreamExt, Stream};
use rodio::{Source, OutputStream};
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stream = get_stream().await?;
    play(stream).await
}

async fn get_stream() -> anyhow::Result<impl Stream<Item = Event>> {
    let transport = web3::transports::WebSocket::new("wss://mainnet.cloudwalk.io/ws").await?;
    let web3 = web3::Web3::new(transport);

    let subscriber = web3.eth_subscribe();
    let filter = web3::types::FilterBuilder::default()
        .from_block(web3::types::BlockNumber::Latest)
        .build();

    let blocks = subscriber
        .subscribe_new_heads()
        .await?
        .filter_map(log_error)
        .map(|_| Event::Kick);

    let logs = subscriber
        .subscribe_logs(filter)
        .await?
        .filter_map(log_error)
        .map(|_| Event::Hat);

    Ok(blocks.merge(logs))
}

async fn play<S>(mut stream: S) -> anyhow::Result<()>
where
    S: Stream<Item = Event> + Unpin
{
    let samples = Samples::load()?;
    // Get a output stream handle to the default physical sound device
    let (_stream, output) = OutputStream::try_default()?;

    let mut last_hat = Instant::now();

    while let Some(event) = stream.next().await {
        match event {
            Event::Kick => {
                output.play_raw(samples.kicks[2].decoder()?.convert_samples())?;

                output.play_raw(
                    samples.kicks[0]
                        .decoder()?
                        .delay(Duration::from_millis(500))
                        .convert_samples(),
                )?;

                output.play_raw(
                    samples.snares[0]
                        .decoder()?
                        .delay(Duration::from_millis(470))
                        .convert_samples(),
                )?;
            },
            Event::Snare => todo!(),
            Event::Hat => {
                if last_hat.elapsed().as_millis() > 200 {
                    output.play_raw(
                        samples.hats[0]
                            .decoder()?
                            .delay(Duration::from_millis(250))
                            .convert_samples(),
                    )?;

                    output.play_raw(
                        samples.hats[2]
                            .decoder()?
                            .delay(Duration::from_millis(750))
                            .convert_samples(),
                    )?;
                }

                last_hat = Instant::now();
            },
        }
    }

    Ok(())

}

fn log_error<T, E>(result: Result<T, E>) -> Option<T>
where
    E: std::error::Error,
{
    match result {
        Ok(value) => Some(value),
        Err(error) => {
            eprintln!("error: {}", error);
            None
        },
    }
}
