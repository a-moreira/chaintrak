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
                let soundscape = &samples
                    .soundscapes
                    .choose(&mut rng)
                    .context("no soundscape")?;
                output.play_raw(soundscape.decoder()?.convert_samples())?;
            },

            Event::PixCashier => {
                let badger = samples.badgers.choose(&mut rng).context("no badger")?;
                output.play_raw(badger.decoder()?.convert_samples())?;
            },

            Event::SpinMachine => {
                let ambience = samples.ambiences.choose(&mut rng).context("no ambiances")?;
                output.play_raw(ambience.decoder()?.convert_samples())?;
            },

            Event::Brlc => {
                log::info!("😌🎹");
                let synth = &samples.synths.choose(&mut rng).context("no synths")?;
                output.play_raw(synth.decoder()?.convert_samples())?;
            },

            Event::Compound => {
                let pad = &samples.pads.choose(&mut rng).context("no pads")?;
                output.play_raw(pad.decoder()?.convert_samples())?;
            },
        }
    }

    Ok(())
}

