use super::Step;
use colored::Colorize;
use std::process::Command;

/// executes the given steps, returns the not executed steps in case of an issue
pub fn execute(steps: Vec<Step>) -> Vec<Step> {
    let mut steps_iter = steps.into_iter();
    while let Some(step) = steps_iter.next() {
        println!("\n\n{}\n", step.to_string().bold());
        let mut command = Command::new(step.command);
        command.args(step.args);
        let success = match command.status() {
            Ok(status) => status.success(),
            _ => false,
        };
        if !success {
            return steps_iter.collect();
        }
    }
    vec![]
}
