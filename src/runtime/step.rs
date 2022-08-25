use super::Operation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Step {
    pub step_number: usize,
    pub operation: Operation,
}
