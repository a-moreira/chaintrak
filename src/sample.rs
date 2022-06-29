use std::{
    convert::AsRef,
    fs,
    sync::Arc,
    io
};

use rodio::{decoder::DecoderError, Decoder};

#[derive(Debug, Clone)]
pub struct Sample(Arc<Vec<u8>>);

impl AsRef<[u8]> for Sample {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Sample {
    pub fn load(filename: &str) -> io::Result<Sample> {
        let data = fs::read(filename)?;
        Ok(Self(Arc::new(data)))
    }

    pub fn cursor(&self) -> io::Cursor<Self> {
        io::Cursor::new(self.clone())
    }

    pub fn decoder(&self) -> Result<Decoder<io::Cursor<Sample>>, DecoderError> {
        Decoder::new(self.cursor())
    }
}

#[derive(Debug, Clone)]
pub struct Samples {
    pub kick: Sample,
    pub hat: Sample,
}

impl Samples {
    pub fn load() -> io::Result<Self> {
        let kick = Sample::load("assets/kick.ogg")?;
        let hat = Sample::load("assets/hat.ogg")?;
        Ok(Self { kick, hat })
    }
}
