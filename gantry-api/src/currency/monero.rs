use axum::{
    extract::{FromRef, State},
    http::StatusCode,
    Json,
};
use gantry::api::ProvisionAccountResponse;
use monero_rpc::{RpcClientBuilder, WalletClient};
use redb::TableDefinition;
use tracing::{debug, info, instrument};

use crate::AppState;

#[derive(Clone, Debug)]
pub struct MoneroState {
    wallet: WalletClient,
}

impl MoneroState {
    pub async fn new() -> anyhow::Result<Self> {
        debug!("Connecting to wallet RPC");
        let wallet = RpcClientBuilder::new()
            .rpc_authentication(monero_rpc::RpcAuthentication::Credentials {
                username: "monero".to_string(),
                password: "".to_string(),
            })
            .build("http://127.0.0.1:1234")?
            .wallet();

        info!(
            block_height = wallet.get_height().await?,
            "Connected to wallet RPC"
        );
        Ok(Self { wallet })
    }
}

// TODO proper rate limiting
#[instrument(ret)]
pub async fn provision(
    State(state): State<AppState>,
) -> Result<Json<ProvisionAccountResponse>, StatusCode> {
    // Generate a new subaddress
    let (address, _) = state.monero.wallet.create_address(0, None).await.unwrap();

    // Add id to lifetime table
    let write_txn = state.db.begin_write().unwrap();
    {
        let mut table = write_txn
            .open_table(TableDefinition::<String, u64>::new("lifetime"))
            .unwrap();
        table.insert(address.as_hex(), 0).unwrap();
    }
    write_txn.commit().unwrap();

    Ok(Json(ProvisionAccountResponse {
        address: address.as_hex(),
    }))
}

pub async fn scan(State(state): State<AppState>) -> Result<(), StatusCode> {
    // let address_data = state.monero.wallet.get_address(0, None).await?;

    state
        .monero
        .wallet
        .incoming_transfers(monero_rpc::TransferType::Available, Some(0), None)
        .await
        .unwrap();
    Ok(())
}
