use crate::error::UserError;
use std::env;

#[derive(Eq, PartialEq)]
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
    /// Displays usage instructions
    Help,
    /// Skips the current workflow step and executes the next one
    Ignore,
    /// Skips all workflow steps that fail
    IgnoreAll,
    /// Lists all subfolders matching the given condition
    List { cmd: String, args: Vec<String> },
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
    /// Manually visit each sibling after the current directory
    WalkFromHere,
}

pub fn parse(args: &mut env::Args) -> Result<Command, UserError> {
    let _binary_name = args.next(); // skip the binary name
    let Some(cmd) = args.next() else {
        return Err(help("no command provided"));
    };
    Ok(match cmd.as_str() {
        "abort" => Command::Abort,
        "activate" => Command::Activate,
        "all" => Command::All,
        "clone" => Command::Clone {
            org: args.next().ok_or_else(|| help("no org provided"))?,
        },
        "except" => Command::Except {
            cmd: args.next().ok_or_else(|| help("no executable provided"))?,
            args: args.collect(),
        },
        "help" => Command::Help,
        "ignore" => Command::Ignore,
        "ignore-all" => Command::IgnoreAll,
        "list" => Command::List {
            cmd: args.next().ok_or_else(|| help("no executable provided"))?,
            args: args.collect(),
        },
        "next" => Command::Next,
        "only" => Command::Only {
            cmd: args.next().ok_or_else(|| help("no executable provided"))?,
            args: args.collect(),
        },
        "retry" => Command::Retry,
        "run" => Command::Run {
            cmd: args.next().ok_or_else(|| help("no executable provided"))?,
            args: args.collect(),
        },
        "status" => Command::Status,
        "walk" => Command::Walk { start: None },
        "walk-from" => Command::Walk {
            start: Some(args.next().ok_or(UserError::MissingStartFolder)?),
        },
        "walk-from-here" => Command::WalkFromHere,
        other => return Err(help(format!("unknown command: {}", other))),
    })
}

fn help<IS: Into<String>>(error: IS) -> UserError {
    UserError::WrongCliArguments {
        message: error.into(),
    }
}
