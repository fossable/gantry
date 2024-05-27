use anyhow::Result;
use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use clap::Parser;
use redb::{Database, TableDefinition};
use std::{env, process::ExitCode, sync::Arc};
use tracing::info;

pub mod currency;
pub mod hosting;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CommandLine {
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

#[tokio::main]
async fn main() -> Result<ExitCode> {
    let args = CommandLine::parse();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Parse bind address/port
    // TODO

    let state = AppState {
        db: Arc::new(Database::create(format!(
            "{}/app.db",
            args.datadir.unwrap_or("/tmp".to_string())
        ))?),

        #[cfg(feature = "monero")]
        monero: crate::currency::monero::MoneroState::new().await?,
    };

    let app = Router::new();

    #[cfg(feature = "monero")]
    let app = app.route("/xmr/provision", post(crate::currency::monero::provision));

    info!("Starting listener");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app.with_state(state)).await?;
    Ok(ExitCode::SUCCESS)
}
