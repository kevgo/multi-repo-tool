use crate::error::UserError;
use crate::runtime::Step;

pub fn next(previous_steps: Vec<Step>) -> Result<Vec<Step>, UserError> {
    if previous_steps.is_empty() {
        return Err(UserError::NoNextFolder {});
    }
    Ok(previous_steps)
}
