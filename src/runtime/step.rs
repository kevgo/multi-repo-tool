use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
    },
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
