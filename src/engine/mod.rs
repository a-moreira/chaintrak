mod ambient;
mod jazz;
mod sample;

use rodio::OutputStream;
use tokio_stream::Stream;

use crate::events::Event;
use self::sample::Samples;

#[derive(Debug, Clone, Copy)]
pub enum Program {
    Ambient,
    Jazz,
}

impl Program {
    pub async fn play<S>(&self, events: S) -> anyhow::Result<()>
    where
        S: Stream<Item = Event> + Unpin,
    {
        let samples = Samples::load()?;
        // Get a output stream handle to the default physical sound device
        let (_stream, output) = OutputStream::try_default()?;

        match self {
            Program::Jazz => jazz::play(&samples, events, &output).await,
            Program::Ambient => ambient::play(&samples, events, &output).await,
        }
    }

}
