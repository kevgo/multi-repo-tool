use super::Step;
use colored::Colorize;
use std::env;
use std::process::Command;

pub enum Outcome {
    /// exit in the middle of execution
    Exit { remaining_steps: Vec<Step> },
    /// all steps were successfully executed
    Success,
    /// the given step has failed
    StepFailed {
        failed_step: Step,
        exit_code: u8,
        remaining_steps: Vec<Step>,
    },
}

/// executes the given steps, returns the not executed steps in case of an issue
pub fn execute(steps: Vec<Step>) -> Outcome {
    let mut steps_iter = steps.into_iter();
    while let Some(step) = steps_iter.next() {
        println!("\n\n{}\n", step.to_string().bold());
        let result = match &step {
            Step::Run {
                id: _,
                command,
                args,
            } => run_command(command, args),
            Step::Chdir { id: _, dir } => change_wd(dir),
            Step::Exit { id: _ } => Err(0),
        };
        if let Err(exit_code) = result {
            let mut remaining_steps = vec![step.clone()];
            remaining_steps.extend(steps_iter);
            return if exit_code == 0 {
                Outcome::Exit { remaining_steps }
            } else {
                Outcome::StepFailed {
                    exit_code,
                    failed_step: step,
                    remaining_steps,
                }
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
