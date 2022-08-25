mod cli;
mod commands;
mod github;
mod operations;
mod runtime;

use clap::StructOpt;
use cli::Command;
use runtime::{delete, save};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Arguments::parse();
    let persisted_steps = runtime::load();
    let current_steps = match args.command {
        Command::Abort => commands::abort(&persisted_steps),
        Command::Clone { org } => commands::clone(&org),
        Command::Ignore => commands::ignore(persisted_steps),
        Command::Retry => commands::retry(persisted_steps),
    };
    let leftover_steps = runtime::run(current_steps);
    if leftover_steps.len() > 0 {
        println!("Abort, Retry, Ignore?");
        save(&leftover_steps)
    } else {
        delete()
    }
}
