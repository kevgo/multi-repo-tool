use super::Step;
use std::env;
use std::process::Command;

pub enum Outcome {
    /// exit in the middle of execution
    Exit {
        exit_dir: String,
        remaining_steps: Vec<Step>,
    },
    /// all steps were successfully executed
    Success,
    /// the given step has failed
    StepFailed {
        exit_code: u8,
        remaining_steps: Vec<Step>,
        exit_dir: String,
    },
}

/// executes the given steps, returns the not executed steps in case of an issue
pub fn execute(steps: Vec<Step>) -> Outcome {
    let mut steps_iter = steps.into_iter();
    while let Some(step) = steps_iter.next() {
        println!("\n\n");
        match &step {
            Step::Run { id, cmd, args } => println!("step {}: run {} {}", id, cmd, args.join(" ")),
            Step::Chdir { id, dir } => println!("step {}: cd {}", id, dir),
            Step::Exit { id: _ } => {}
        }
        let result = match &step {
            Step::Run { id: _, cmd, args } => run_command(cmd, args),
            Step::Chdir { id: _, dir } => change_wd(dir),
            Step::Exit { id: _ } => {
                let current_dir = env::current_dir().expect("cannot determine current directory");
                return Outcome::Exit {
                    exit_dir: current_dir.to_string_lossy().to_string(),
                    remaining_steps: steps_iter.collect(),
                };
            }
        };
        if let Err(exit_code) = result {
            let current_dir = env::current_dir().expect("cannot determine current directory");
            let mut remaining_steps = vec![step];
            remaining_steps.extend(steps_iter);
            return Outcome::StepFailed {
                exit_code,
                remaining_steps,
                exit_dir: current_dir.to_string_lossy().to_string(),
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
