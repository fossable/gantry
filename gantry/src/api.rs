use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Currency {
    Xmr,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProvisionAccountResponse {
    /// Account ID (wallet address)
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentRatesResponse {
    pub currency: Currency,
    pub hour: u64,
    pub day: u64,
    pub month: u64,
    pub year: u64,
}
