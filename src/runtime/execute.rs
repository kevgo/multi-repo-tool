use super::Step;
use crate::config::Config;
use colored::Colorize;
use std::env;
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
    Success { config: Config },
    /// the given step has failed
    StepFailed {
        /// error code
        code: u8,
        /// remaining steps
        config: Config,
        /// subfolder in which the problem happened
        dir: String,
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
    let mut steps_iter = config.steps.into_iter();
    while let Some(step) = steps_iter.next() {
        let text = match &step {
            Step::Run { id, cmd, args } => format!("step {}: run {} {}", id, cmd, args.join(" ")),
            Step::Chdir { id, dir } => format!("step {}: cd {}", id, dir),
            Step::Exit { id: _ } => "".into(),
        };
        println!("\n{}", text.bold());
        let result = match &step {
            Step::Run { id: _, cmd, args } => run_command(cmd, args, ignore_all),
            Step::Chdir { id: _, dir } => change_wd(dir),
            Step::Exit { id: _ } => {
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
        if let Err(exit_code) = result {
            let current_dir = env::current_dir().expect("cannot determine current directory");
            let mut remaining_steps = vec![step];
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

pub fn change_wd(dir: &str) -> Result<(), u8> {
    match env::set_current_dir(dir) {
        Ok(_) => Ok(()),
        Err(_) => Err(1),
    }
}

/// executes the given command in the current working directory
pub fn run_command(cmd: &str, args: &Vec<String>, ignore_all: bool) -> Result<(), u8> {
    let mut command = Command::new(cmd);
    command.args(args);
    let status = command.status().expect("cannot determine exit status");
    let exit_code = status.code().expect("cannot determine exit code");
    if exit_code > 0 && !ignore_all {
        return Err(exit_code as u8);
    }
    Ok(())
}
