use crate::config::Config;
use crate::error::UserError;
use crate::helpers::println::println_bold;
use colored::Colorize;
use std::env;
use std::path::PathBuf;

pub fn abort(config: Config) -> Result<Config, UserError> {
    if config.steps.is_empty() {
        return Err(UserError::NothingToAbort {});
    }
    println_bold!("aborting {} steps", config.steps.len());
    if let Some(dir) = config.root_dir {
        let cwd = env::current_dir().expect("cannot determine current directory");
        if cwd != PathBuf::from(&dir) {
            println_bold!("returning to {}\n", &dir);
            env::set_current_dir(dir).expect("cannot cd into the initial directory");
        }
    }
    Ok(Config {
        steps: vec![],
        root_dir: None,
        ..config
    })
}
