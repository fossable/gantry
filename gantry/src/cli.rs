#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Provision a new account
    Provision {},
    Deprovision {},
}
