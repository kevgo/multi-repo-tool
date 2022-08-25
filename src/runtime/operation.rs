use std::fmt::Display;
use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::runtime::save;

pub fn clone(id: usize, url: String) -> Step {
    Step {
        id,
        command: "git".into(),
        args: vec!["clone".into(), url],
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Step {
    pub id: usize,
    pub command: String,
    pub args: Vec<String>,
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.id, self.command, self.args.join(" "))
    }
}

pub fn run(steps: Vec<Step>) {
    let mut steps_iter = steps.into_iter();
    while let Some(step) = steps_iter.next() {
        println!("{}", step);
        let mut command = Command::new(step.command);
        command.args(step.args);
        let success = match command.status() {
            Ok(status) => status.success(),
            _ => false,
        };
        if !success {
            save(steps_iter).unwrap();
            return;
        }
    }
}
