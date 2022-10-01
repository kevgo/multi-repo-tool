use std::process::ExitCode;

use crate::config::Config;
use crate::error::UserError;

pub fn retry(config: Config) -> Result<(Config, Option<ExitCode>), UserError> {
    if config.steps.is_empty() {
        return Err(UserError::NothingToRetry {});
    }
    Ok((config, None))
}
