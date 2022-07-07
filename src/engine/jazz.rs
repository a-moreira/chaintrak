use anyhow::Context;
use rand::prelude::SliceRandom;
use rodio::{Source, OutputStreamHandle};
use tokio_stream::{Stream, StreamExt};

use crate::events::Event;
use super::sample::Samples;

pub async fn play<S>(
    samples: &Samples,
    mut events: S,
    output: &OutputStreamHandle
) -> anyhow::Result<()>
where
    S: Stream<Item = Event> + Unpin,
{
    let mut rng = rand::thread_rng();

    while let Some(event) = events.next().await {
        match event {
            Event::Block => {
                let jazz_loop = samples
                    .jazz_loops
                    .choose(&mut rng)
                    .context("no jazz loops")?;

                output.play_raw(jazz_loop.decoder()?.convert_samples())?;
            }

            Event::PixCashier => {
                log::info!("epic sax guy 🎷");
                let sax = &samples.saxes.choose(&mut rng).context("no saxes")?;
                output.play_raw(sax.decoder()?.convert_samples())?;
            }
            Event::SpinMachine => {
                let piano = &samples.pianos.choose(&mut rng).context("no pianos")?;
                output.play_raw(piano.decoder()?.convert_samples())?;
            }
            Event::Brlc => {
                let bass = samples.basses.choose(&mut rng).context("no basses")?;

                output.play_raw(bass.decoder()?.convert_samples())?;
            }
            Event::Compound => {
                let perc = &samples
                    .percussions
                    .choose(&mut rng)
                    .context("no percussion")?;
                output.play_raw(perc.decoder()?.convert_samples())?;
            }
        };
    }

    Ok(())
}
