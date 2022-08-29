mod cli;
mod commands;
mod error;
mod helpers;
mod runtime;

use camino::Utf8PathBuf;
use clap::StructOpt;
use cli::Command;
use colored::Colorize;
use error::UserError;
use runtime::{dir_file, step_queue, Outcome};
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
    let initial_dir = env::current_dir().expect("cannot determine the current directory");
    let initial_dir = Utf8PathBuf::from_path_buf(initial_dir).expect("invalid unicode in filename");
    let config_dir = match step_queue::location() {
        Some(dir) => dir,
        None => initial_dir,
    };
    let persisted_steps = step_queue::load(config_dir)?;
    let current_steps = match args.command {
        Command::Abort => commands::abort(&persisted_steps)?,
        Command::Clone { org } => commands::clone(&org),
        Command::Run { cmd, args } => commands::run(&cmd, &args, &initial_dir)?,
        Command::Ignore => commands::ignore(persisted_steps)?,
        Command::Next | Command::Retry => commands::retry(persisted_steps)?,
        Command::Walk => commands::walk(&initial_dir)?,
    };
    match runtime::execute(current_steps) {
        Outcome::Success => {
            step_queue::delete()?;
            Ok(())
        }
        Outcome::StepFailed { code, steps, dir } => {
            step_queue::save(&initial_dir, &steps)?;
            dir_file::save(&initial_dir, &dir)?;
            Err(UserError::StepFailed { code })
        }
        Outcome::Exit { steps, dir } => {
            step_queue::save(&initial_dir, &steps)?;
            dir_file::save(&initial_dir, &dir)?;
            Ok(())
        }
    }
}
