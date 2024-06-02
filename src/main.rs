use crate::cli::Commands;
use anyhow::Result;
use clap::Parser;
use std::process::ExitCode;

pub mod cli;
pub mod currency;
pub mod hosting;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CommandLine {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[tokio::main]
async fn main() -> Result<ExitCode> {
    let args = CommandLine::parse();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Dispatch command
    match &args.command {
        Some(Commands::Provision {}) => todo!(),
        Some(Commands::Deprovision {}) => todo!(),
        Some(Commands::Serve(args)) => crate::cli::serve::serve(args).await,
        None => todo!(),
    }
}
