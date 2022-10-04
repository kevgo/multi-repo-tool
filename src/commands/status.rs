use crate::config::Config;
use crate::error::UserError;
use crate::helpers::{folder_list, subdirs};
use std::process::ExitCode;

pub fn status(config: &Config) -> Result<(Config, Option<ExitCode>), UserError> {
    let all_count = match &config.root_dir {
        Some(root_dir) => Some(subdirs::count(root_dir)?),
        None => None,
    };
    match &config.folders {
        Some(folders) => {
            match all_count {
                Some(all) => println!("Running only in {}/{} folders:", folders.len(), all),
                None => println!("Running only in {} folders:", folders.len()),
            }
            folder_list::print(folders);
        }
        None => match all_count {
            Some(all) => println!("Running in all {} folders.", all),
            None => println!("Running in all folders."),
        },
    }
    println!();
    if config.steps.is_empty() {
        println!("I'm not doing anything right now.");
    } else {
        for step in &config.steps {
            println!("{}", step);
        }
    }
    Ok((Config::default(), Some(ExitCode::SUCCESS)))
}
