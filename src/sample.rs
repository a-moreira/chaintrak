use rodio::{decoder::DecoderError, Decoder};
use std::{convert::AsRef, fs, io, path::Path, sync::Arc};

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
    pub shakers: Vec<Sample>,
    pub percussions: Vec<Sample>,
    pub synths: Vec<Sample>,
    pub pads: Vec<Sample>,
    pub jazz_loops: Vec<Sample>,
    pub basses: Vec<Sample>,
    pub ambiances: Vec<Sample>,
    pub saxes: Vec<Sample>,
    pub pianos: Vec<Sample>,
}

impl Samples {
    pub fn load() -> io::Result<Self> {
        let shakers = Self::load_files("assets/synth-ep-*.ogg")?;
        let percussions = Self::load_files("assets/synth-jazz-*.ogg")?;
        let synths = Self::load_files("assets/synth-jazz-*.ogg")?;
        let pads = Self::load_files("assets/synth-jazz-*.ogg")?;
        let jazz_loops = Self::load_files("assets/jazz-drums-*.ogg")?;
        let basses = Self::load_files("assets/bass-*.ogg")?;
        let ambiances = Self::load_files("assets/synth-badger-*.ogg")?;
        let saxes = Self::load_files("assets/sax-*.ogg")?;
        let pianos = Self::load_files("assets/synth-ep-*.ogg")?;
        Ok(Self {
            shakers,
            percussions,
            synths,
            pads,
            jazz_loops,
            basses,
            ambiances,
            saxes,
            pianos,
        })
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
