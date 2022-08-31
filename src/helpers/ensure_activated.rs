use crate::error::UserError;
use std::env;

pub fn ensure_activated() -> Result<(), UserError> {
    for (name, value) in env::vars() {
        if name == "MRT_ACTIVATED" && value == "true" {
            return Ok(());
        }
    }
    Err(UserError::NotActivated)
}
