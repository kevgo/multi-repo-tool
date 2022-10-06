use crate::config::Config;
use crate::error::UserError;
use std::process::ExitCode;

#[allow(clippy::unnecessary_wraps)]
pub fn status(config: &Config) -> Result<(Config, Option<ExitCode>), UserError> {
    println!("{}", config);
    Ok((Config::default(), Some(ExitCode::SUCCESS)))
}
