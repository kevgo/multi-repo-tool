use clap;

/// the CLI commands that could be run
#[derive(clap::StructOpt)]
#[clap(version, about, long_about = None)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    Abort,
    Clone { org: String },
    Ignore,
    Retry,
}
