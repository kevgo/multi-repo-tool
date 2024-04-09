use crate::error::UserError;
use std::env;

pub fn ensure_activated() -> Result<(), UserError> {
  if let Ok(value) = env::var("MRT_WRAPPED") {
    if value == "true" {
      return Ok(());
    }
  }
  Err(UserError::NotWrapped)
}
