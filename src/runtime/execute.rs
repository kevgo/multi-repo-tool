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
pub fn execute(config: Config, ignore_all: bool) -> Outcome {
    if config.steps.is_empty() {
        return Outcome::Success {
            config: Config {
                root_dir: None,
                ..config
            },
        };
    }

    // somehow this is enough to ensure a graceful exit
    ctrlc::set_handler(move || {
        println!(" Canceling the current step...");
    })
    .expect("Error setting Ctrl-C handler");

    let mut steps_iter = config.steps.into_iter();
    let mut last_dir: Option<String> = None;
    while let Some(numbered) = steps_iter.next() {
        let text = match &numbered.step {
            Step::Run { cmd, args } => {
                if args.is_empty() {
                    format!("step {}: run {}", numbered.id, cmd)
                } else {
                    format!("step {}: run {} {}", numbered.id, cmd, args.join(" "))
                }
            }
            Step::Chdir { dir } => format!("step {}: cd {}", numbered.id, dir),
            Step::Exit => "".into(),
        };
        println!("\n{}", text.bold());
        let result = match &numbered.step {
            Step::Run { cmd, args } => run_command(cmd, args, ignore_all),
            Step::Chdir { dir } => {
                last_dir = Some(dir.into());
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
                let mut remaining_steps = vec![
                    NumberedStep {
                        id: numbered.id - 1,
                        step: Step::Chdir {
                            dir: last_dir.unwrap(),
                        },
                    },
                    numbered,
                ];
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
    println!("\n{}\n", "ALL DONE".bold());
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
