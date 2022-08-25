mod cli;
mod commands;
mod github;
mod operations;
mod runtime;

use clap::StructOpt;
use cli::Command;
use runtime::{delete, save};

fn main() {
    let args = cli::Arguments::parse();
    let previous_steps = runtime::load();
    let steps = match args.command {
        Command::Abort => commands::abort(&previous_steps),
        Command::Clone { org } => commands::clone(&org),
        Command::Ignore => commands::ignore(previous_steps),
        Command::Retry => commands::retry(previous_steps),
    };
    let leftover_steps = runtime::run(steps);
    if leftover_steps.len() > 0 {
        save(&leftover_steps).unwrap();
    } else {
        delete();
    }
}
