use ethers::types::*;

#[derive(Debug, Clone, Copy)]
pub struct BalanceChange {
    pub address: H160,
    pub amount_0_in: U256,
    pub amount_1_in: U256,
    pub amount_0_out: U256,
    pub amount_1_out: U256,
}