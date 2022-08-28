/// Runs CLI commands for all repositories of a Github organization.
#[derive(clap::StructOpt)]
#[clap(version, about, long_about = None)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// Deletes the currently running workflow
    Abort,
    /// Clones a Github organization into the current directory
    Clone { org: String },
    /// Skips the current workflow step and executes the next one
    Ignore,
    /// Continues the currently running workflow by retrying the last failed step
    Retry,
    /// Executes the given CLI command in all repositories
    Run { cmd: String, args: Vec<String> },
    /// Manually visits each subdirectory
    Walk,
}
