use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub enum Step {
    Run {
        id: u32,
        cmd: String,
        args: Vec<String>,
    },
    Chdir {
        id: u32,
        dir: String,
    },
    Exit {
        id: u32,
    },
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Step::Run { id, cmd, args } => {
                write!(f, "step {}: {} {}", id, cmd, args.join(" "))
            }
            Step::Chdir { id, dir } => write!(f, "step {}: cd {}", id, dir),
            Step::Exit { id } => write!(f, "step {}: exit", id),
        }
    }
}
