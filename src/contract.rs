use web3::types::H160;

pub struct Contract {
    pub address: H160,
}

impl Contract {
    pub fn new(contract_address: [u8; 20]) -> anyhow::Result<Contract> {
        let address = web3::types::H160(contract_address);

        Ok(Self { address })
    }

    pub fn create_log_filter(self) -> anyhow::Result<web3::types::Filter> {
        let log_filter = web3::types::FilterBuilder::default()
            .address(vec![self.address])
            .build();

        Ok(log_filter)
    }
}
