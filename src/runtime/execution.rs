use super::Step;
use colored::Colorize;
use std::process::Command;

pub enum Outcome {
    Success,
    StepFailed {
        exitCode: i32,
        remainingSteps: Vec<Step>,
    },
}

/// executes the given steps, returns the not executed steps in case of an issue
pub fn execute(steps: Vec<Step>) -> Outcome {
    let mut steps_iter = steps.into_iter();
    while let Some(step) = steps_iter.next() {
        println!("\n\n{}\n", step.to_string().bold());
        let mut command = Command::new(step.command);
        command.args(step.args);
        if let Ok(status) = command.status() {
            if let Some(exitCode) = status.code() {
                if exitCode > 0 {
                    return Outcome::StepFailed {
                        exitCode,
                        remainingSteps: steps_iter.collect(),
                    };
                }
            }
        };
    }
    Outcome::Success
}
