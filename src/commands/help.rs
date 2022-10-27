use crate::config::Config;
use colored::Colorize;
use std::process::ExitCode;

pub fn help() -> (Config, Option<ExitCode>) {
    println!("{}", "Usage: m <command>\n".bold());
    println!(
        "Don't call mrt directly. Run {}, then call the shell function {}.\n",
        "mrt activate | source".bold(),
        "m".bold()
    );
    println!(
        "To execute a CLI command in all subfolders: {}",
        "m run <executable> [<arguments>]".bold()
    );
    println!(
        "If the given executable fails in one of the subfolders, you end up in that subfolder."
    );
    println!("After investigating/fixing the failure, you can:");
    println!(
        "{}        stops iterating the remaining subfolders",
        "m abort".bold()
    );
    println!("{}       retries the failed command", "m retry".bold());
    println!(
        "{}      ignores this subfolder and continues in the next subfolder",
        "m ignore".bold()
    );
    println!(
        "{}  ignores all subsequent failures in all subfolders\n",
        "m ignore-all".bold()
    );
    println!(
        "To open a command prompt in all subfolders: {}",
        "m walk".bold()
    );
    println!(
        "When you are done with one subfolder, run {} to go to the next subfolder.",
        "m next".bold()
    );
    println!("To stop the process early: {}", "m abort".bold());
    println!(
        "To start walking at a specific subfolder: {}",
        "m walk-from <folder name>".bold()
    );
    (Config::default(), Some(ExitCode::SUCCESS))
}
