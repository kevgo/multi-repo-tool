mod cli;
mod commands;
mod error;
mod github;
mod operations;
mod runtime;

use std::process::ExitCode;

use clap::StructOpt;
use cli::Command;
use error::UserError;
use runtime::Outcome;

fn main() -> ExitCode {
    match main2() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            println!("Error: {}", err);
            err.exit_code()
        }
    }
}

fn main2() -> Result<(), UserError> {
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
            runtime::forget()?;
            Ok(())
        }
        Outcome::StepFailed {
            exit_code,
            failed_step,
            remaining_steps,
        } => {
            runtime::persist(&remaining_steps)?;
            Err(UserError::StepFailed {
                step: failed_step,
                exit_code: exit_code as u8,
            })
        }
    }
}
