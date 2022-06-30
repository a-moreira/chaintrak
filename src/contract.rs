use web3::types::H160;
use hex_literal::hex;

pub struct Contract {
    pub address: H160,
}

impl Contract {
    pub fn new(self, contract_address: &str) -> Result<Contract, anyhow::Error> {
        let hexed = hex!(contract_address);
        let address = web3::types::H160(hexed);

        Ok(Self { address })
    }

    pub fn create_log_filter(self) -> Result<web3::types::Filter, anyhow::Error> {
        let log_filter = web3::types::FilterBuilder::default()
            .address(vec!(self.address))
            .build();

        Ok(log_filter)
    }
}
