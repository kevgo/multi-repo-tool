use crate::cli;
use crate::config::Config;
use crate::error::UserError;
use crate::runtime::steps::{NumberedStep, Step};
use colored::Colorize;
use std::env;
use std::io::ErrorKind;
use std::process::Command;

pub enum Outcome {
    /// exit in the middle of execution
    Exit {
        /// the folder into which to exit
        dir: String,
        /// the remaining steps
        config: Config,
    },
    /// all steps were successfully executed
    Success {
        config: Config,
    },
    /// the given step has failed
    StepFailed {
        /// error code
        code: u8,
        /// remaining steps
        config: Config,
        /// subfolder in which the problem happened
        dir: String,
    },
    UserError {
        error: UserError,
    },
}

/// executes the given steps, returns the not executed steps in case of an issue
pub fn execute(config: Config, command: &cli::Command) -> Outcome {
    // somehow this is enough to ensure a graceful exit
    ctrlc::set_handler(move || {
        println!(" Canceling the current step...");
    })
    .expect("Error setting Ctrl-C handler");

    let max_step = config.steps.last().map_or(0, |step| step.id);
    let mut steps_iter = config.steps.into_iter();
    let mut previous_dir: Option<NumberedStep> = None;
    while let Some(numbered) = steps_iter.next() {
        print_step(&numbered, max_step);
        let result = match &numbered.step {
            Step::Run { cmd, args } => run_command(cmd, args, command == &cli::Command::IgnoreAll),
            Step::Chdir { dir } => {
                previous_dir = Some(numbered.clone());
                change_wd(dir)
            }
            Step::Exit => {
                let current_dir = env::current_dir().expect("cannot determine current directory");
                return Outcome::Exit {
                    dir: current_dir.to_string_lossy().to_string(),
                    config: Config {
                        steps: steps_iter.collect(),
                        ..config
                    },
                };
            }
        };
        match result {
            Ok(exit_code) if exit_code == 0 => {}
            Ok(exit_code) => {
                let current_dir = env::current_dir().expect("cannot determine current directory");
                let mut remaining_steps = vec![previous_dir.unwrap(), numbered];
                remaining_steps.extend(steps_iter);
                return Outcome::StepFailed {
                    code: exit_code,
                    config: Config {
                        steps: remaining_steps,
                        ..config
                    },
                    dir: current_dir.to_string_lossy().to_string(),
                };
            }
            Err(user_error) => return Outcome::UserError { error: user_error },
        }
    }
    if let Some(dir) = config.root_dir {
        env::set_current_dir(dir).expect("cannot cd into the initial directory");
    }
    match command {
        cli::Command::Clone { org: _ }
        | cli::Command::Ignore
        | cli::Command::IgnoreAll
        | cli::Command::Next
        | cli::Command::Retry
        | cli::Command::Walk { start: _ }
        | cli::Command::Run { cmd: _, args: _ } => {
            println!("\n{}\n", "ALL DONE".bold());
        }
        cli::Command::Abort
        | cli::Command::Activate
        | cli::Command::All
        | cli::Command::Except { cmd: _, args: _ }
        | cli::Command::Help
        | cli::Command::Only { cmd: _, args: _ }
        | cli::Command::Status => {}
    }
    Outcome::Success {
        config: Config {
            steps: vec![],
            root_dir: None,
            ..config
        },
    }
}

pub fn change_wd(dir: &str) -> Result<u8, UserError> {
    match env::set_current_dir(dir) {
        Ok(_) => Ok(0),
        Err(err) => Err(UserError::CannotChangeIntoDirectory {
            dir: dir.into(),
            guidance: err.to_string(),
        }),
    }
}

fn print_step(numbered: &NumberedStep, max: u32) {
    let text = match &numbered.step {
        Step::Run { cmd, args } => {
            if args.is_empty() {
                format!("step {}/{}: run {}", numbered.id, max, cmd)
            } else {
                format!(
                    "step {}/{}: run {} {}",
                    numbered.id,
                    max,
                    cmd,
                    args.join(" ")
                )
            }
        }
        Step::Chdir { dir } => format!("step {}/{}: cd {}", numbered.id, max, dir),
        Step::Exit => "".into(),
    };
    println!("\n{}", text.bold());
}

/// executes the given command in the current working directory
pub fn run_command(cmd: &str, args: &Vec<String>, ignore_all: bool) -> Result<u8, UserError> {
    let mut command = Command::new(cmd);
    command.args(args);
    match command.status() {
        Ok(status) => {
            if status.success() || ignore_all {
                Ok(0)
            } else {
                Ok(status.code().unwrap_or(1) as u8)
            }
        }
        Err(err) => match err.kind() {
            ErrorKind::NotFound => Err(UserError::CommandNotFound {
                command: cmd.into(),
            }),
            ErrorKind::PermissionDenied => Err(UserError::ExecutePermissionDenied {
                command: cmd.into(),
            }),
            _ => Err(UserError::OtherExecutionError {
                command: cmd.into(),
                guidance: err.to_string(),
            }),
        },
    }
}
