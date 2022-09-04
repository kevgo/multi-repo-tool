use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NumberedStep {
    pub id: u32,
    pub step: Step,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum Step {
    Run { cmd: String, args: Vec<String> },
    Chdir { dir: String },
    Exit,
}

impl Display for NumberedStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.step {
            Step::Run { cmd, args } => {
                write!(f, "step {}: {} {}", self.id, cmd, args.join(" "))
            }
            Step::Chdir { dir } => write!(f, "step {}: cd {}", self.id, dir),
            Step::Exit => write!(f, "step {}: exit", self.id),
        }
    }
}

pub fn numbered(steps: Vec<Step>) -> Vec<NumberedStep> {
    steps
        .into_iter()
        .enumerate()
        .map(|(id, step)| NumberedStep {
            id: id as u32,
            step,
        })
        .collect()
}
