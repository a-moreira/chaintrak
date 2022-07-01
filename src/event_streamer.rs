use std::pin::Pin;

use hex_literal::hex;
use tokio_stream::{Stream, StreamMap, StreamExt};

use crate::contract::Contract;


#[derive(Debug, Copy, Clone)]
pub enum Event {
    Block,
    Brlc,
    PixCashier,
    SpinMachine,
}

pub async fn start() -> anyhow::Result<impl Stream<Item = Event>> {
    let brlc_address = hex!("A9a55a81a4C085EC0C31585Aed4cFB09D78dfD53");
    let pix_cashier_address = hex!("c8eb60d121EF768C94438a7F0a38AADfC401f301");
    let spin_machine_address = hex!("4F05d2E56B868361D2C8Bbd51B662C78296018A8");

    let brlc_filter = Contract::new(brlc_address)?.create_log_filter()?;
    let pix_cashier_filter = Contract::new(pix_cashier_address)?.create_log_filter()?;
    let spin_machine_filter = Contract::new(spin_machine_address)?.create_log_filter()?;

    let transport = web3::transports::WebSocket::new("wss://mainnet.cloudwalk.io/ws").await?;
    let web3 = web3::Web3::new(transport);
    let subscriber = web3.eth_subscribe();

    let mut stream = StreamMap::<_, Pin<Box<dyn Stream<Item = Event>>>>::new();

    stream.insert(
        "block",
        Box::pin(
            subscriber
                .subscribe_new_heads()
                .await?
                .filter_map(log_error)
                .map(|_| Event::Block)
        )
    );

    stream.insert(
        "brlc",
        Box::pin(
            subscriber
                .subscribe_logs(brlc_filter)
                .await?
                .filter_map(log_error)
                .map(|_| Event::Brlc)
        )
    );

    stream.insert(
        "pix-cashier",
        Box::pin(
            subscriber
                .subscribe_logs(pix_cashier_filter)
                .await?
                .filter_map(log_error)
                .map(|_| Event::PixCashier)
        )
    );

    stream.insert(
        "spin-machine",
        Box::pin(
            subscriber
                .subscribe_logs(spin_machine_filter)
                .await?
                .filter_map(log_error)
                .map(|_| Event::SpinMachine)
        )
    );

    Ok(stream.map(|(_, event)| event))
}

fn log_error<T, E>(result: Result<T, E>) -> Option<T>
where
    E: std::error::Error,
{
    result
        .map_err(|error| log::error!("{}", error))
        .ok()
}
