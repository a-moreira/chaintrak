use std::{sync::mpsc::{Receiver, TryRecvError}, thread, time::Duration};

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
        let mut ambience = false;
        let mut soundscape = false;
        let mut synth = false;
        let mut pad = false;
        let mut badger  = false;
        loop {
            match events.try_recv() {
                Ok(Event::Block) => soundscape = true,
                Ok(Event::PixCashier) => badger = true,
                Ok(Event::SpinMachine) => synth = true,
                Ok(Event::Brlc) => pad = true,
                Ok(Event::Compound) => ambience = true,
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return Ok(()),
            };
        }

        if ambience {
            let ambience = samples.ambiences.choose(&mut rng).context("no ambiances")?;
            output.play_raw(ambience.decoder()?.convert_samples())?;
        }

        if synth {
            log::info!("😌🎹");
            let synth = &samples.synths.choose(&mut rng).context("no synths")?;
            output.play_raw(synth.decoder()?.convert_samples())?;
        }

        if soundscape {
            let soundscape = &samples
                .soundscapes
                .choose(&mut rng)
                .context("no soundscape")?;
            output.play_raw(soundscape.decoder()?.convert_samples())?;
        }

        if pad {
            let pad = &samples.pads.choose(&mut rng).context("no pads")?;
            output.play_raw(pad.decoder()?.convert_samples())?;
        }

        if badger {
            let badger = samples.badgers.choose(&mut rng).context("no badger")?;
            output.play_raw(badger.decoder()?.convert_samples())?;
        }

        thread::sleep(Duration::from_millis(30));
    }
}

