mod cli;
mod commands;
mod github;
mod operations;
mod runtime;

use clap::StructOpt;
use cli::Command;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Arguments::parse();
    let persisted_steps = runtime::load()?;
    let current_steps = match args.command {
        Command::Abort => commands::abort(&persisted_steps),
        Command::Clone { org } => commands::clone(&org),
        Command::Ignore => commands::ignore(persisted_steps),
        Command::Retry => commands::retry(persisted_steps),
    };
    let leftover_steps = runtime::execute(current_steps);
    if leftover_steps.is_empty() {
        runtime::forget();
        Ok(())
    } else {
        println!("Abort, Retry, Ignore?");
        runtime::persist(&leftover_steps)
    }
}
