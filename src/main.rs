use cloudrave::{contract::Contract, sample::Samples, Event};
use tokio_stream::{StreamExt, Stream};
use rodio::{Source, OutputStream};
use std::time::{Duration, Instant};
use hex_literal::hex;
use rand::Rng;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stream = get_stream().await?;
    play(stream).await
}

async fn get_stream() -> anyhow::Result<impl Stream<Item = Event>> {
    let transport = web3::transports::WebSocket::new("wss://mainnet.cloudwalk.io/ws").await?;
    let web3 = web3::Web3::new(transport);
    let subscriber = web3.eth_subscribe();

    let brlc_address: &str = "A9a55a81a4C085EC0C31585Aed4cFB09D78dfD53";
    let pix_cashier_address: &str = "c8eb60d121EF768C94438a7F0a38AADfC401f301";
    let spin_machine_address: &str ="4F05d2E56B868361D2C8Bbd51B662C78296018A8";
    let brlc_filter = Contract::new(brlc_address)::create_log_filter();
    let pix_cashier_filter = Contract::new(pix_cashier_address)::create_log_filter();
    let spin_machine_filter = Contract::new(spin_machine_address)::create_log_filter();

    let blocks = subscriber
        .subscribe_new_heads()
        .await?
        .filter_map(log_error)
        .map(|_| Event::Kick);

    let brlc_logs = subscriber
        .subscribe_logs(brlc_filter)
        .await?
        .filter_map(log_error)
        .map(|_| Event::Shaker);

    let pix_cashier_logs = subscriber
        .subscribe_logs(pix_cashier_filter)
        .await?
        .filter_map(log_error)
        .map(|_| Event::ClosedHat);

    let spin_machine_logs = subscriber
        .subscribe_logs(spin_machine_filter)
        .await?
        .filter_map(log_error)
        .map(|_| Event::Percussion);

    let logs = brlc_logs.merge(pix_cashier_logs);

    Ok(blocks.merge(logs))
}

async fn play<S>(mut stream: S) -> anyhow::Result<()>
where
    S: Stream<Item = Event> + Unpin
{
    let samples = Samples::load()?;
    // Get a output stream handle to the default physical sound device
    let (_stream, output) = OutputStream::try_default()?;

    let mut last_closed_hat = Instant::now();

    let mut rng = rand::thread_rng();

    while let Some(event) = stream.next().await {
        match event {
            Event::Kick => {
                output.play_raw(samples.kicks[2].decoder()?.convert_samples())?;

                output.play_raw(
                    samples.kicks[2]
                        .decoder()?
                        .delay(Duration::from_millis(500))
                        .convert_samples(),
                )?;

            },
            Event::Snare => todo!(),
            Event::ClosedHat => {
                output.play_raw(samples.closed_hats[0].decoder()?.convert_samples())?;
            },
            Event::Percussion => {
                output.play_raw(samples.percussions[0].decoder()?.convert_samples())?;
            },
            Event::Shaker => {
                output.play_raw(
                    samples.shakers[0]
                        .decoder()?
                        .delay(Duration::from_millis(rng.gen_range(200..500)))
                        .convert_samples(),
                )?;

                output.play_raw(
                    samples.shakers[2]
                        .decoder()?
                        .delay(Duration::from_millis(500))
                        .convert_samples(),
                )?;
            }
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
