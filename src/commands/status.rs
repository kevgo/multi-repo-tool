use crate::config::Config;
use std::process;

pub fn status(config: Config) -> Config {
    match config.folders {
        Some(folders) => {
            println!("Running only in these folders:");
            for folder in folders {
                println!("- {}", folder);
            }
        }
        None => println!("Running in all folders."),
    }
    if config.steps.is_empty() {
        println!("I'm not doing anything right now.");
    } else {
        for step in config.steps {
            println!("{}", step);
        }
    }
    process::exit(0);
}
