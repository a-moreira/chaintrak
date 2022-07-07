pub mod streamer;
mod contract;

#[derive(Debug, Copy, Clone)]
pub enum Event {
    Block,
    Brlc,
    PixCashier,
    SpinMachine,
    Compound,
}
