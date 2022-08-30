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
    let current_steps = match args.command {
        Command::Abort => commands::abort(&persisted_steps)?,
        Command::Clone { org } => commands::clone(&org),
        Command::Run { cmd, args } => commands::run(&cmd, &args, &initial_dir)?,
        Command::Ignore => commands::ignore(persisted_steps)?,
        Command::Next => commands::next(persisted_steps)?,
        Command::Retry => commands::retry(persisted_steps)?,
        Command::Walk => commands::walk(&initial_dir)?,
    };
    match runtime::execute(current_steps) {
        Outcome::Success => {
            step_queue::delete(&config_path)?;
            let cwd = env::current_dir().expect("cannot determine current dir");
            if cwd != initial_dir {
                dir_file::save(initial_dir, &cwd.to_string_lossy())?;
            }
            Ok(())
        }
        Outcome::StepFailed { code, steps, dir } => {
            step_queue::save(config_path, &steps)?;
            dir_file::save(initial_dir, &dir)?;
            Err(UserError::StepFailed { code })
        }
        Outcome::Exit { steps, dir } => {
            step_queue::save(config_path, &steps)?;
            dir_file::save(initial_dir, &dir)?;
            Ok(())
        }
    }
}
