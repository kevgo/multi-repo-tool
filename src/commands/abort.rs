use crate::config::Config;
use crate::error::UserError;
use std::env;

pub fn abort(config: Config) -> Result<Config, UserError> {
    if config.steps.is_empty() {
        return Err(UserError::NothingToAbort {});
    }
    println!("aborting {} steps", config.steps.len());
    if let Some(dir) = config.root_dir {
        println!("returning to the {} dir", &dir);
        env::set_current_dir(dir).expect("cannot cd into the initial directory");
    }
    Ok(Config {
        steps: vec![],
        root_dir: None,
        ..config
    })
}
