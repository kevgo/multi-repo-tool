use serde::{Deserialize, Serialize};

/// the different operations that this tool can do
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Operation {
    CloneRepo { name: String, url: String },
}
