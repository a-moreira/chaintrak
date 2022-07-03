use std::{
    sync::mpsc::{Receiver, TryRecvError},
    thread,
    time::Duration,
};

use anyhow::Context;
use rand::prelude::SliceRandom;
use rodio::{OutputStream, Source};
use tokio_stream::{Stream, StreamExt};

use crate::{event_streamer::Event, sample::Samples};

pub async fn play<S>(mut events: S, mode: String) -> anyhow::Result<()>
where
    S: Stream<Item = Event> + Unpin,
{
    let (sender, receiver) = std::sync::mpsc::channel();
    thread::spawn(move || {
        let result = match mode.as_str() {
            "jazz" => play_jazz(receiver),
            "ambient" => play_ambient(receiver),
            _ => panic!("pick one: jazz or ambient"),
        };

        if let Err(error) = result {
            log::error!("{}", error);
        }
    });

    while let Some(event) = events.next().await {
        sender.send(event)?;
    }

    Ok(())
}

fn play_jazz(events: Receiver<Event>) -> anyhow::Result<()> {
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
            let sax = &samples.saxes.choose(&mut rng).context("no saxes")?;
            output.play_raw(sax.decoder()?.convert_samples())?;
        }

        if bass {
            let bass = samples.basses.choose(&mut rng).context("no basses")?;

            output.play_raw(bass.decoder()?.convert_samples())?;
        }

        thread::sleep(Duration::from_millis(250));
    }
}

fn play_ambient(events: Receiver<Event>) -> anyhow::Result<()> {
    let samples = Samples::load()?;
    // Get a output stream handle to the default physical sound device
    let (_stream, output) = OutputStream::try_default()?;

    let mut rng = rand::thread_rng();

    loop {
        let mut ambiance = false;
        let mut synth = false;
        let mut pad = false;
        let mut percussion = false;
        let mut bass = false;
        loop {
            match events.try_recv() {
                Ok(Event::Block) => ambiance = true,
                Ok(Event::PixCashier) => pad = true,
                Ok(Event::SpinMachine) => synth = true,
                Ok(Event::Brlc) => bass = true,
                Ok(Event::Compound) => percussion = true,
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return Ok(()),
            };
        }

        if ambiance {
            let ambiance = samples.ambiances.choose(&mut rng).context("no ambiances")?;

            output.play_raw(ambiance.decoder()?.convert_samples())?;
        }

        if synth {
            let synth = &samples.synths.choose(&mut rng).context("no synths")?;
            output.play_raw(synth.decoder()?.convert_samples())?;
        }

        if percussion {
            let perc = &samples
                .percussions
                .choose(&mut rng)
                .context("no percussion")?;
            output.play_raw(perc.decoder()?.convert_samples())?;
        }

        if pad {
            let pad = &samples.pads.choose(&mut rng).context("no pads")?;
            output.play_raw(pad.decoder()?.convert_samples())?;
        }

        if bass {
            let bass = samples.basses.choose(&mut rng).context("no basses")?;

            output.play_raw(bass.decoder()?.convert_samples())?;

            // output.play_raw(
            //     bass
            //         .decoder()?
            //         .delay(Duration::from_millis(500))
            //         .convert_samples(),
            // )?;
        }

        thread::sleep(Duration::from_millis(250));
    }
}
