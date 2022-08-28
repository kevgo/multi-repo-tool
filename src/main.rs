mod cli;
mod commands;
mod error;
mod github;
mod helpers;
mod runtime;

use camino::Utf8PathBuf;
use clap::StructOpt;
use cli::Command;
use colored::Colorize;
use error::UserError;
use runtime::Outcome;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    match inner() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            println!("\n{}{}\n", "Error: ".red().bold(), err.to_string().red());
            err.exit_code()
        }
    }
}

fn inner() -> Result<(), UserError> {
    let args = cli::Arguments::parse();
    let persisted_steps = runtime::load()?;
    let initial_dir = env::current_dir().expect("cannot determine the current directory");
    let initial_dir = Utf8PathBuf::from_path_buf(initial_dir).expect("invalid unicode in filename");
    let current_steps = match args.command {
        Command::Abort => commands::abort(&persisted_steps)?,
        Command::Clone { org } => commands::clone(&org),
        Command::Run { cmd, args } => commands::run(&cmd, &args, &initial_dir)?,
        Command::Ignore => commands::ignore(persisted_steps)?,
        Command::Retry => commands::retry(persisted_steps)?,
        Command::Walk => commands::walk(&initial_dir)?,
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
            runtime::persist(&initial_dir, &remaining_steps)?;
            Err(UserError::StepFailed {
                step: failed_step,
                exit_code,
            })
        }
    }
}
