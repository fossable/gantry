use crate::cli::Commands;
use clap::Parser;

mod cli;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CommandLine {
    #[clap(subcommand)]
    command: Option<Commands>,
}

fn main() {
    // Parse command line options before we configure logging so we can set the
    // default level
    let command_line = CommandLine::parse();

    // Configure logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Dispatch command
    match &command_line.command {
        Some(Commands::Provision {}) => todo!(),
        Some(Commands::Deprovision {}) => todo!(),
        None => todo!(),
    }
}
