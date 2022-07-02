use std::{time::Duration, sync::mpsc::{Receiver, TryRecvError}, thread};

use anyhow::Context;
use rand::prelude::SliceRandom;
use rodio::{OutputStream, Source};
use tokio_stream::{Stream, StreamExt};

use crate::{event_streamer::Event, sample::Samples};

pub async fn play<S>(mut events: S, mode: String) -> anyhow::Result<()>
where
    S: Stream<Item = Event> + Unpin
{
    let (sender, receiver) = std::sync::mpsc::channel();
    thread::spawn(move || {
        let result = match mode.as_str() {
            "rave" => drummer(receiver),
            "relax" => todo!(),
            _  => panic!("pick one: rave or relax"),
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

fn drummer(events: Receiver<Event>) -> anyhow::Result<()> {
    let samples = Samples::load()?;
    // Get a output stream handle to the default physical sound device
    let (_stream, output) = OutputStream::try_default()?;

    let mut rng = rand::thread_rng();

    loop {
        let mut kick = false;
        let mut shaker = false;
        let mut percussion = false;
        let mut hat = false;
        loop {
            match events.try_recv() {
                Ok(Event::Block) => kick = true,
                Ok(Event::PixCashier) => hat = true,
                Ok(Event::SpinMachine) => percussion = true,
                Ok(Event::Brlc) => shaker = true,
                Ok(Event::Compound) => todo!(),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return Ok(()),
            };
        }

        if kick {
            let kick = samples.kicks.choose(&mut rng).context("no kicks")?;

            output.play_raw(kick.decoder()?.convert_samples())?;

            output.play_raw(
                kick
                    .decoder()?
                    .delay(Duration::from_millis(500))
                    .convert_samples(),
            )?;
        }

        if hat {
            let hat = &samples.hats.choose(&mut rng).context("no hats")?;
            output.play_raw(hat.decoder()?.convert_samples())?;
        }

        if percussion {
            let perc = &samples.percussions.choose(&mut rng).context("no percussion")?;
            output.play_raw(perc.decoder()?.convert_samples())?;
        }

        if shaker {
            let shaker = samples.shakers.choose(&mut rng).context("no shakers")?;

            output.play_raw(
                shaker
                    .decoder()?
                    .convert_samples(),
            )?;

            output.play_raw(
                shaker
                    .decoder()?
                    .delay(Duration::from_millis(500))
                    .convert_samples(),
            )?;
        }

        thread::sleep(Duration::from_millis(250));
    };
}
