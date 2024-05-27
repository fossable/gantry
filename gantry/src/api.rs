use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProvisionAccountResponse {
    /// Account ID (wallet address)
    pub address: String,
}
