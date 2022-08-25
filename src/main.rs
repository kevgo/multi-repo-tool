mod cli;
mod commands;
mod github;
mod operations;

use clap::StructOpt;

fn main() {
    let args = cli::Arguments::parse();
    match args.command {
        cli::Command::Clone { org } => commands::clone(&org),
    };
}
