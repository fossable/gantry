use anyhow::Result;
use std::collections::HashMap;

#[cfg(feature = "monero")]
pub mod monero;

pub async fn lookup() -> Result<HashMap<String, f64>> {
    let mapping: HashMap<String, f64> =
        reqwest::get("https://min-api.cryptocompare.com/data/price?fsym=USD&tsyms=XMR")
            .await?
            .json()
            .await?;

    Ok(mapping)
}
