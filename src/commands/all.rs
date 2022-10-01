use crate::config::Config;
use std::process::ExitCode;

pub fn all(config: Config) -> (Config, Option<ExitCode>) {
    (
        Config {
            folders: None,
            ..config
        },
        None,
    )
}
