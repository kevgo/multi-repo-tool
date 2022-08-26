use crate::error::UserError;
use crate::runtime::Step;

pub fn retry(previous_steps: Vec<Step>) -> Result<Vec<Step>, UserError> {
    if previous_steps.is_empty() {
        return Err(UserError::NothingToRetry {});
    }
    Ok(previous_steps)
}
