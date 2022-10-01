use std::process::ExitCode;

use crate::config::Config;

pub fn all(config: Config) -> (Config, Option<ExitCode>) {
    (
        Config {
            folders: None,
            ..config
        },
        None,
    )
}
