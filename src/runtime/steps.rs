use serde::{Deserialize, Serialize};

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
    Check { cmd: String, args: Vec<String> },
    Exit,
}

impl NumberedStep {
    pub fn list(&self, max_id: u32) -> String {
        match &self.step {
            Step::Run { cmd, args } => {
                format!("step {}/{}: {} {}", self.id, max_id, cmd, args.join(" "))
            }
            Step::Chdir { dir } => format!("step {}/{}: cd {}", self.id, max_id, dir),
            Step::Check { cmd, args } => format!(
                "step {}/{}: check {} {}",
                self.id,
                max_id,
                cmd,
                args.join(" ")
            ),
            Step::Exit => format!("step {}/{}: exit", self.id, max_id),
        }
    }
}

pub fn numbered(steps: Vec<Step>) -> Vec<NumberedStep> {
    let mut numbered_steps = vec![];
    for (i, step) in steps.into_iter().enumerate() {
        numbered_steps.push(NumberedStep {
            #[allow(clippy::cast_possible_truncation)]
            id: i as u32 + 1,
            step,
        });
    }
    numbered_steps
}
