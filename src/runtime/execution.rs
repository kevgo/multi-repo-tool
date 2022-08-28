use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::Display;
use std::process::Command;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Step {
    Run {
        id: u32,
        command: String,
        args: Vec<String>,
    },
    Chdir {
        id: u32,
        dir: String,
    },
    Exit {
        id: u32,
    }
}

impl Step {
    pub fn execute(&self) -> Result<(), u8> {
        match self {
            Step::Run {
                id: _,
                command,
                args,
            } => run_command(command, args),
            Step::Chdir { id: _, dir } => change_wd(dir),
            Step::Exit { id: _ } => Err(0),
        }
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Step::Run { id, command, args } => {
                write!(f, "step {}: {} {}", id, command, args.join(" "))
            }
            Step::Chdir { id, dir } => write!(f, "step {}: cd {}", id, dir),
            Step::Exit { id } => write!(f, "step {}: exit", id),
        }
    }
}

pub enum Outcome {
    Success,
    StepFailed {
        exit_code: u8,
        failed_step: Step,
        remaining_steps: Vec<Step>,
    },
}

/// executes the given steps, returns the not executed steps in case of an issue
pub fn execute(steps: Vec<Step>) -> Outcome {
    let mut steps_iter = steps.into_iter();
    while let Some(step) = steps_iter.next() {
        println!("\n\n{}\n", step.to_string().bold());
        if let Err(exit_code) = step.execute() {
            let mut remaining_steps = vec![step.clone()];
            remaining_steps.extend(steps_iter);
            return Outcome::StepFailed {
                exit_code,
                failed_step: step,
                remaining_steps,
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
