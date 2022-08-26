use super::Step;
use colored::Colorize;
use std::process::Command;

pub enum Outcome {
    Success,
    StepFailed {
        exit_code: i32,
        failed_step: Step,
        remaining_steps: Vec<Step>,
    },
}

/// executes the given steps, returns the not executed steps in case of an issue
pub fn execute(steps: Vec<Step>) -> Outcome {
    let mut steps_iter = steps.into_iter();
    while let Some(step) = steps_iter.next() {
        println!("\n\n{}\n", step.to_string().bold());
        let mut command = Command::new(&step.command);
        command.args(&step.args);
        if let Ok(status) = command.status() {
            if let Some(exit_code) = status.code() {
                if exit_code > 0 {
                    return Outcome::StepFailed {
                        failed_step: step,
                        exit_code,
                        remaining_steps: steps_iter.collect(),
                    };
                }
            }
        };
    }
    Outcome::Success
}
