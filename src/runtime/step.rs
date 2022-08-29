use super::{change_wd, run_command};
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
