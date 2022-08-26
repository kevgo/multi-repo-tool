mod cli;
mod commands;
mod github;
mod operations;
mod runtime;

use clap::StructOpt;
use cli::Command;
use runtime::Outcome;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Arguments::parse();
    let persisted_steps = runtime::load()?;
    let current_steps = match args.command {
        Command::Abort => commands::abort(&persisted_steps),
        Command::Clone { org } => commands::clone(&org),
        Command::Ignore => commands::ignore(persisted_steps),
        Command::Retry => commands::retry(persisted_steps),
    };
    match runtime::execute(current_steps) {
        Outcome::Success => {
            runtime::forget();
            Ok(())
        }
        Outcome::StepFailed {
            exitCode,
            remainingSteps,
        } => {
            println!("Abort, Retry, Ignore?");
            runtime::persist(&remainingSteps)
        }
    }
}
