use crate::config::Config;
use crate::error::UserError;
use crate::helpers::get_subdirs;
use crate::helpers::println::println_bold;
use camino::Utf8Path;
use colored::Colorize;
use std::process::Command;

/// defines which folders get included
pub enum Mode {
    /// include folders that match the given condition
    Match,
    /// include folders that don't match the given condition
    NoMatch,
}

pub fn limit(
    cmd: &str,
    args: &[String],
    root_dir: &Utf8Path,
    mode: &Mode,
) -> Result<Config, UserError> {
    let mut folders = vec![];
    for dir in get_subdirs(root_dir)? {
        let mut command = Command::new(&cmd);
        command.args(args);
        command.current_dir(&dir);
        if let Ok(status) = command.status() {
            let push = match mode {
                Mode::Match => status.success(),
                Mode::NoMatch => !status.success(),
            };
            if push {
                folders.push(dir);
            }
        }
    }
    if folders.is_empty() {
        return Err(UserError::NoFoldersToIterate);
    }
    println_bold!("\n{}", "Execution has been limited to these folders:");
    for (i, folder) in folders.iter().enumerate() {
        println!("{}. {}", i + 1, folder);
    }
    Ok(Config {
        folders: Some(folders),
        ..Config::default()
    })
}
