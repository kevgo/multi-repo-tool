use crate::config::Config;
use colored::Colorize;
use std::process::ExitCode;

pub fn help() -> (Config, Option<ExitCode>) {
    println!("{}", "Usage: m <command>\n".bold());
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
    println!(
        "By default, mrt iterates through all direct subfolders of the directory it is called."
    );
    println!("To limit the set of folders that mrt goes through:");
    println!(
        "m only <condition>   keeps only the folders for which condition returns exit code 0."
    );
    println!("You can call \"m only\" repeatedly to limit by multiple criteria.");
    println!("m unfold <condition> replaces the current folder set with all subfolders of the current folders");
    println!("for which the given condition returns exit code 0.");
    (Config::default(), Some(ExitCode::SUCCESS))
}
