use crate::config::Config;
use crate::error::UserError;

pub fn abort(config: Config) -> Result<Config, UserError> {
    if config.steps.is_empty() {
        return Err(UserError::NothingToAbort {});
    }
    println!("aborting {} steps", config.steps.len());
    Ok(Config {
        steps: vec![],
        ..config
    })
}
