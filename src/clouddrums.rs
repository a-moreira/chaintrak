use std::time::Duration;

use rand::Rng;
use rodio::{OutputStream, Source};
use tokio_stream::{Stream, StreamExt};

use crate::{event_streamer::Event, sample::Samples};

pub async fn play<S>(mut events: S) -> anyhow::Result<()>
where
    S: Stream<Item = Event> + Unpin
{
    let samples = Samples::load()?;
    // Get a output stream handle to the default physical sound device
    let (_stream, output) = OutputStream::try_default()?;

    let mut rng = rand::thread_rng();

    while let Some(event) = events.next().await {
        match event {
            Event::Block => {
                output.play_raw(samples.kicks[2].decoder()?.convert_samples())?;

                output.play_raw(
                    samples.kicks[2]
                        .decoder()?
                        .delay(Duration::from_millis(500))
                        .convert_samples(),
                )?;

            },
            Event::PixCashier => {
                output.play_raw(samples.closed_hats[0].decoder()?.convert_samples())?;
            },
            Event::SpinMachine => {
                output.play_raw(samples.percussions[0].decoder()?.convert_samples())?;
            },
            Event::Brlc => {
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
