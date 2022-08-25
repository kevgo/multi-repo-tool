mod cli;
mod commands;
mod github;
mod operations;
mod runtime;

use clap::StructOpt;
use cli::Command;

fn main() {
    let args = cli::Arguments::parse();
    let previous_steps = runtime::load();
    let steps = match args.command {
        Command::Abort => commands::abort(&previous_steps),
        Command::Clone { org } => commands::clone(&org),
        Command::Ignore => commands::ignore(previous_steps),
        Command::Retry => commands::retry(previous_steps),
    };
    runtime::run(steps);
}
