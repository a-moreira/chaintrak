mod ambient;
mod jazz;

use std::thread;

use tokio_stream::{Stream, StreamExt};

use crate::event_streamer::Event;

#[derive(Debug, Clone, Copy)]
pub enum Program {
    Ambient,
    Jazz,
}

pub async fn play<S>(mut events: S, program: Program) -> anyhow::Result<()>
where
    S: Stream<Item = Event> + Unpin,
{
    let (sender, receiver) = std::sync::mpsc::channel();
    thread::spawn(move || {
        let result = match program {
            Program::Jazz => jazz::play(receiver),
            Program::Ambient => ambient::play(receiver),
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
