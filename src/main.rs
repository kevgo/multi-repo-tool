mod cli;
mod commands;
mod config;
mod error;
mod helpers;
mod runtime;

use crate::helpers::println::println_error;
use camino::Utf8PathBuf;
use clap::StructOpt;
use cli::Command;
use colored::Colorize;
use config::Config;
use error::UserError;
use runtime::{dir_file, Outcome};
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    match inner() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            println!();
            println_error!("{}\n", err);
            err.exit_code()
        }
    }
}

fn inner() -> Result<(), UserError> {
    let cli_args = cli::Arguments::parse();
    if cli_args.command != Command::Activate {
        helpers::ensure_activated()?;
    }
    let initial_dir = env::current_dir().expect("cannot determine the current directory");
    let initial_dir = Utf8PathBuf::from_path_buf(initial_dir).expect("invalid unicode current dir");
    let config_path = config::filepath();
    let persisted_config = config::load(&config_path)?;
    let ignore_all = cli_args.command == Command::IgnoreAll;
    let config_to_execute = match cli_args.command {
        Command::Abort => commands::abort(persisted_config)?,
        Command::Activate => commands::activate(),
        Command::All => commands::all(persisted_config),
        Command::Clone { org } => commands::clone(&org, initial_dir.to_string()),
        Command::Run { cmd, args } => commands::run(&cmd, &args, persisted_config, &initial_dir)?,
        Command::Ignore | Command::IgnoreAll => commands::ignore(persisted_config)?,
        Command::Limit { cmd, args } => commands::limit(&cmd, &args, &initial_dir)?,
        Command::Next => commands::next(persisted_config)?,
        Command::Retry => commands::retry(persisted_config)?,
        Command::Status => commands::status(persisted_config),
        Command::Walk => commands::walk(&initial_dir, persisted_config)?,
    };
    match runtime::execute(config_to_execute, ignore_all) {
        Outcome::Success { config } => {
            config::save(
                &config_path,
                &Config {
                    steps: vec![],
                    root_dir: None,
                    ..config
                },
            )?;
            let cwd = env::current_dir().expect("cannot determine current dir");
            if cwd != initial_dir {
                dir_file::save(&cwd.to_string_lossy())?;
            }
            Ok(())
        }
        Outcome::StepFailed { code, config, dir } => {
            config::save(&config_path, &config)?;
            dir_file::save(&dir)?;
            Err(UserError::StepFailed { code })
        }
        Outcome::Exit { config, dir } => {
            config::save(&config_path, &config)?;
            dir_file::save(&dir)?;
            Ok(())
        }
    }
}
