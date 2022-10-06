/// Runs CLI commands for all repositories of a Github organization.
#[derive(clap::StructOpt)]
#[clap(version, about, long_about = None)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Clone, Eq, PartialEq)]
pub enum Command {
    /// Deletes the currently running workflow
    Abort,
    /// Enable shell integration,
    Activate,
    /// Removes a previously set limit
    All,
    /// Clones a Github organization into the current directory
    Clone { org: String },
    /// Limits activities to a subset of subfolders that don't match the given criteria
    Except { cmd: String, args: Vec<String> },
    /// Skips the current workflow step and executes the next one
    Ignore,
    /// Skips all workflow steps that fail
    IgnoreAll,
    /// Limits activities to a subset of subfolders that match the given criteria
    Only { cmd: String, args: Vec<String> },
    /// Goes to the next subdirectory during walk
    Next,
    /// Continues the currently running workflow by retrying the last failed step
    Retry,
    /// Executes the given CLI command in all repositories
    Run { cmd: String, args: Vec<String> },
    /// Displays the current status of the command queue
    Status,
    /// Manually visits each subdirectory, optionally starting at the given one
    Walk { start: Option<String> },
}
