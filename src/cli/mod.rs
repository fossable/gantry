use self::serve::ServeArgs;

pub mod serve;

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Commands {
    /// Provision a new account
    Provision {},
    Deprovision {},
    Serve(ServeArgs),
}
