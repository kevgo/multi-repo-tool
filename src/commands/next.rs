use std::process::ExitCode;

use crate::config::Config;
use crate::error::UserError;

pub fn next(config: Config) -> Result<(Config, Option<ExitCode>), UserError> {
    if config.steps.is_empty() {
        return Err(UserError::NoNextFolder {});
    }
    Ok((config, None))
}
