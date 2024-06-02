use anyhow::Result;
use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use clap::{Args, Parser};
use redb::{Database, TableDefinition};
use std::{env, process::ExitCode, sync::Arc};
use tokio::net::TcpListener;
use tokio::spawn;
use tracing::info;

#[derive(Debug, Clone, Args)]
pub struct ServeArgs {
    bind: Option<String>,
    datadir: Option<String>,
    // #[cfg(feature = "monero")]
    // monero_wallet_url: String,
}

#[derive(Clone, Debug)]
pub struct AppState {
    db: Arc<Database>,

    #[cfg(feature = "monero")]
    monero: crate::currency::monero::MoneroState,
}

pub async fn serve(args: &ServeArgs) -> Result<ExitCode> {
    let state = AppState {
        db: Arc::new(Database::create(format!(
            "{}/app.db",
            args.datadir.as_ref().unwrap_or(&"/tmp".to_string())
        ))?),

        #[cfg(feature = "monero")]
        monero: crate::currency::monero::MoneroState::new(&args).await?,
    };

    let app = Router::new();

    #[cfg(feature = "monero")]
    let app = app.route("/xmr/provision", post(crate::currency::monero::provision));

    info!("Starting listener");
    let listener =
        TcpListener::bind(args.bind.as_ref().unwrap_or(&"0.0.0.0:3000".to_string())).await?;
    axum::serve(listener, app.with_state(state)).await?;
    Ok(ExitCode::SUCCESS)
}
