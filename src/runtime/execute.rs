use super::Step;
use colored::Colorize;
use std::env;
use std::process::Command;

pub enum Outcome {
    /// exit in the middle of execution
    Exit {
        /// the folder into which to exit
        dir: String,
        /// the remaining steps
        steps: Vec<Step>,
    },
    /// all steps were successfully executed
    Success,
    /// the given step has failed
    StepFailed {
        /// error code
        code: u8,
        /// remaining steps
        steps: Vec<Step>,
        /// subfolder in which the problem happened
        dir: String,
    },
}

/// executes the given steps, returns the not executed steps in case of an issue
pub fn execute(steps: Vec<Step>) -> Outcome {
    let mut steps_iter = steps.into_iter();
    while let Some(step) = steps_iter.next() {
        let text = match &step {
            Step::Run { id, cmd, args } => format!("step {}: run {} {}", id, cmd, args.join(" ")),
            Step::Chdir { id, dir } => format!("step {}: cd {}", id, dir),
            Step::Exit { id: _ } => "".into(),
        };
        println!("\n{}", text.bold());
        let result = match &step {
            Step::Run { id: _, cmd, args } => run_command(cmd, args),
            Step::Chdir { id: _, dir } => change_wd(dir),
            Step::Exit { id: _ } => {
                let current_dir = env::current_dir().expect("cannot determine current directory");
                return Outcome::Exit {
                    dir: current_dir.to_string_lossy().to_string(),
                    steps: steps_iter.collect(),
                };
            }
        };
        if let Err(exit_code) = result {
            let current_dir = env::current_dir().expect("cannot determine current directory");
            let mut remaining_steps = vec![step];
            remaining_steps.extend(steps_iter);
            return Outcome::StepFailed {
                code: exit_code,
                steps: remaining_steps,
                dir: current_dir.to_string_lossy().to_string(),
            };
        }
    }
    Outcome::Success
}

pub fn change_wd(dir: &str) -> Result<(), u8> {
    match env::set_current_dir(dir) {
        Ok(_) => Ok(()),
        Err(_) => Err(1),
    }
}

/// executes the given command in the current working directory
pub fn run_command(cmd: &str, args: &Vec<String>) -> Result<(), u8> {
    let mut command = Command::new(cmd);
    command.args(args);
    if let Ok(status) = command.status() {
        if let Some(exit_code) = status.code() {
            if exit_code > 0 {
                return Err(exit_code as u8);
            }
        }
    }
    Ok(())
}
