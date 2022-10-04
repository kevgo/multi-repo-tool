use crate::config::Config;
use crate::error::UserError;
use colored::Colorize;
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn abort(config: Config) -> Result<(Config, Option<ExitCode>), UserError> {
    if config.steps.is_empty() {
        return Err(UserError::NothingToAbort {});
    }
    println!(
        "{}",
        format!("aborting {} steps", config.steps.len()).bold()
    );
    if let Some(dir) = &config.root_dir {
        let cwd = env::current_dir().expect("cannot determine current directory");
        if cwd != PathBuf::from(&dir) {
            println!("{}", format!("returning to {}\n", &dir).bold());
            env::set_current_dir(dir).expect("cannot cd into the initial directory");
        }
    }
    Ok((
        Config {
            steps: vec![],
            ..config
        },
        None,
    ))
}
