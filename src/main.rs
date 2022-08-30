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
    let config_path = match step_queue::location(&initial_dir) {
        Some(dir) => dir,
        None => initial_dir.join(step_queue::FILENAME),
    };
    let persisted_steps = step_queue::load(&config_path)?;
    let root_dir = config_path.parent().expect("cannot determine root dir");
    let current_steps = match args.command {
        Command::Abort => commands::abort(&persisted_steps)?,
        Command::Clone { org } => commands::clone(&org),
        Command::Run { cmd, args } => commands::run(&cmd, &args, &initial_dir)?,
        Command::Ignore => commands::ignore(persisted_steps)?,
        Command::Next | Command::Retry => commands::retry(persisted_steps)?,
        Command::Walk => commands::walk(root_dir)?,
    };
    match runtime::execute(current_steps) {
        Outcome::Success => {
            step_queue::delete(&config_path)?;
            Ok(())
        }
        Outcome::StepFailed { code, steps, dir } => {
            step_queue::save(&config_path, &steps)?;
            let current_dir = env::current_dir().expect("cannot determine current dir");
            dir_file::save(&current_dir, &dir)?;
            Err(UserError::StepFailed { code })
        }
        Outcome::Exit { steps, dir } => {
            step_queue::save(&config_path, &steps)?;
            let current_dir = env::current_dir().expect("cannot determine current dir");
            dir_file::save(&current_dir, &dir)?;
            Ok(())
        }
    }
}
