use crate::config::Config;
use crate::error::UserError;
use colored::Colorize;
use std::env;
use std::path::PathBuf;

pub fn abort(config: Config) -> Result<Config, UserError> {
    if config.steps.is_empty() {
        return Err(UserError::NothingToAbort {});
    }
    let text = format!("aborting {} steps", config.steps.len());
    println!("{}", text.bold());
    if let Some(dir) = config.root_dir {
        let cwd = env::current_dir().expect("cannot determine current directory");
        if cwd != PathBuf::from(&dir) {
            let text = format!("returning to {}", &dir);
            println!("{}\n", text.bold());
            env::set_current_dir(dir).expect("cannot cd into the initial directory");
        }
    }
    Ok(Config {
        steps: vec![],
        root_dir: None,
        ..config
    })
}
