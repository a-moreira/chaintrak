pub mod sample;
pub mod contract;

#[derive(Debug, Copy, Clone)]
pub enum Event {
    Kick,
    Snare,
    ClosedHat,
    Shaker,
    Percussion,
}
