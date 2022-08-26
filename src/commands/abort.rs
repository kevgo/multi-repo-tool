use crate::error::UserError;
use crate::runtime::Step;

pub fn abort(previous_steps: &Vec<Step>) -> Result<Vec<Step>, UserError> {
    if previous_steps.is_empty() {
        return Err(UserError::NothingToAbort {});
    }
    println!("aborting {} steps", previous_steps.len());
    Ok(vec![])
}
