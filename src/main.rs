mod cli;
mod commands;
mod github;
mod operations;
mod runtime;

use clap::StructOpt;

fn main() {
    let args = cli::Arguments::parse();
    let steps = match args.command {
        cli::Command::Clone { org } => commands::clone(&org),
    };
    runtime::run(&steps);
}
