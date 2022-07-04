use std::{sync::mpsc::{Receiver, TryRecvError}, time::Duration, thread};

use anyhow::Context;
use rand::prelude::SliceRandom;
use rodio::{OutputStream, Source};

use crate::{event_streamer::Event, sample::Samples};

pub fn play(events: Receiver<Event>) -> anyhow::Result<()> {
    let samples = Samples::load()?;
    // Get a output stream handle to the default physical sound device
    let (_stream, output) = OutputStream::try_default()?;

    let mut rng = rand::thread_rng();

    loop {
        let mut jazz_loop = false;
        let mut piano = false;
        let mut sax = false;
        let mut percussion = false;
        let mut bass = false;
        loop {
            match events.try_recv() {
                Ok(Event::Block) => jazz_loop = true,
                Ok(Event::PixCashier) => sax = true,
                Ok(Event::SpinMachine) => piano = true,
                Ok(Event::Brlc) => bass = true,
                Ok(Event::Compound) => percussion = true,
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return Ok(()),
            };
        }

        if jazz_loop {
            let jazz_loop = samples
                .jazz_loops
                .choose(&mut rng)
                .context("no jazz loops")?;

            output.play_raw(jazz_loop.decoder()?.convert_samples())?;
        }

        if piano {
            let piano = &samples.pianos.choose(&mut rng).context("no pianos")?;
            output.play_raw(piano.decoder()?.convert_samples())?;
        }

        if percussion {
            let perc = &samples
                .percussions
                .choose(&mut rng)
                .context("no percussion")?;
            output.play_raw(perc.decoder()?.convert_samples())?;
        }

        if sax {
            log::info!("epic sax guy 🎷");
            let sax = &samples.saxes.choose(&mut rng).context("no saxes")?;
            output.play_raw(sax.decoder()?.convert_samples())?;
        }

        if bass {
            let bass = samples.basses.choose(&mut rng).context("no basses")?;

            output.play_raw(bass.decoder()?.convert_samples())?;
        }

        // sleep to avoid looping too much
        thread::sleep(Duration::from_millis(20));
    }
}
