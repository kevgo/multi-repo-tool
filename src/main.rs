mod cli;
mod commands;
mod config;
mod error;
mod helpers;
mod runtime;

use camino::Utf8PathBuf;
use cli::Command;
use colored::Colorize;
use commands::Mode;
use config::Config;
use error::UserError;
use runtime::{dir_file, Outcome};
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    match inner() {
        Ok(exit_code) => exit_code,
        Err(err) => {
            let exit_code = err.exit_code();
            let (error, guidance) = err.messages();
            println!("{}{}", "ERROR: ".red().bold(), error.red());
            if !guidance.is_empty() {
                println!("\n{}", guidance);
            }
            exit_code
        }
    }
}

fn inner() -> Result<ExitCode, UserError> {
    let Ok(cli_args) = cli::parse(&mut env::args()) else {
        return Ok(ExitCode::FAILURE);
    };
    if cli_args != Command::Activate && cli_args != Command::Help {
        helpers::ensure_activated()?;
    }
    let init_dir = env::current_dir().expect("cannot determine the current directory");
    let init_dir = Utf8PathBuf::from_path_buf(init_dir).expect("invalid unicode current dir");
    let config_path = config::filepath();
    let persisted_config = config::load(&config_path)?;
    prevent_session_override(&persisted_config, &cli_args)?;
    let (config_to_execute, early_exit) = match &cli_args {
        Command::Abort => commands::abort(persisted_config)?,
        Command::Activate => commands::activate(),
        Command::All => commands::limit::all(persisted_config),
        Command::Clone { org } => commands::clone(org, &init_dir)?,
        Command::Run { cmd, args } => commands::run(cmd, args, persisted_config, &init_dir)?,
        Command::Help => commands::help(),
        Command::Ignore | Command::IgnoreAll => commands::ignore(persisted_config)?,
        Command::List { cmd, args } => commands::list(cmd, args, &init_dir, persisted_config)?,
        Command::Only { cmd, args } => {
            commands::limit::only(cmd, args, &init_dir, &Mode::Match, persisted_config)?
        }
        Command::Except { cmd, args } => {
            commands::limit::only(cmd, args, &init_dir, &Mode::NoMatch, persisted_config)?
        }
        Command::Next => commands::next(persisted_config)?,
        Command::Retry => commands::retry(persisted_config)?,
        Command::Status => commands::status(&persisted_config)?,
        Command::Walk { start } => commands::walk(&init_dir, persisted_config, start.as_ref())?,
        Command::WalkFromHere => commands::walk(
            init_dir.parent().unwrap(),
            persisted_config,
            Some(&init_dir.file_name().unwrap().to_string()),
        )?,
    };
    if let Some(exit_code) = early_exit {
        return Ok(exit_code);
    }
    match runtime::execute(config_to_execute, &cli_args) {
        Outcome::Success { config } => {
            if config == Config::default() {
                config::delete(&config_path);
            } else {
                config::save(
                    &config_path,
                    &Config {
                        steps: vec![],
                        ..config
                    },
                )?;
            }
            let cwd = env::current_dir().expect("cannot determine current dir");
            dir_file::save(&cwd.to_string_lossy())?;
            Ok(ExitCode::SUCCESS)
        }
        Outcome::StepFailed { code, config, dir } => {
            config::save(&config_path, &config)?;
            dir_file::save(&dir)?;
            Err(UserError::StepFailed { code })
        }
        Outcome::Exit { config, dir } => {
            config::save(&config_path, &config)?;
            dir_file::save(&dir)?;
            Ok(ExitCode::SUCCESS)
        }
        Outcome::UserError { error } => Err(error),
    }
}

/// prevents accidental override of an already active session
fn prevent_session_override(config: &Config, command: &Command) -> Result<(), UserError> {
    if config.steps.is_empty() {
        return Ok(());
    }
    match command {
        Command::Run { cmd: _, args: _ } | Command::Walk { start: _ } => {
            Err(UserError::SessionAlreadyActive {
                config: config.clone(),
            })
        }
        _ => Ok(()),
    }
}
