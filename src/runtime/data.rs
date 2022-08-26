use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Step {
    pub id: usize,
    pub command: String,
    pub args: Vec<String>,
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "step {}: {} {}",
            self.id,
            self.command,
            self.args.join(" ")
        )
    }
}
