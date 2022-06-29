pub mod sample;


#[derive(Debug, Copy, Clone)]
pub enum Event {
    Kick,
    Snare,
    ClosedHat,
    Shaker,
    Percussion,
}
