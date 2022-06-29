use std::{
    convert::AsRef,
    fs,
    sync::Arc,
    io, path::Path
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
    pub fn load(filename: &Path) -> io::Result<Sample> {
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
    pub kicks: Vec<Sample>,
    pub closed_hats: Vec<Sample>,
    pub snares: Vec<Sample>,
    pub shakers: Vec<Sample>,
    pub percussions: Vec<Sample>,
}

impl Samples {
    pub fn load() -> io::Result<Self> {
        let kicks = Self::load_files("assets/kick-*.ogg")?;
        let closed_hats = Self::load_files("assets/closed_hat-*.ogg")?;
        let snares = Self::load_files("assets/snare-*.ogg")?;
        let shakers = Self::load_files("assets/shaker-*.ogg")?;
        let percussions = Self::load_files("assets/percussion-*.ogg")?;
        Ok(Self { kicks, closed_hats, snares, shakers, percussions })
    }

    fn load_files(pattern: &str) -> io::Result<Vec<Sample>> {
        let samples = glob::glob(pattern)
            .expect("invalid pattern")
            .filter_map(Result::ok)
            .map(|path| Sample::load(&path))
            .collect::<Result<_, _>>()?;

        Ok(samples)
    }
}
