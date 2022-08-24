mod cli;

use clap::StructOpt;

fn main() {
    let args = cli::Arguments::parse();
    match args.command {
        cli::Command::Clone { org } => {
            println!("Cloning... {}", org)
        }
    }
}
