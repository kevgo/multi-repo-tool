use crate::config::Config;
use crate::error::UserError;
use crate::helpers::get_subdirs;
use crate::helpers::println::println_bold;
use camino::Utf8Path;
use colored::Colorize;
use std::process::Command;

pub fn only(cmd: &str, args: &[String], root_dir: &Utf8Path) -> Result<Config, UserError> {
    let mut folders = vec![];
    for dir in get_subdirs(root_dir)? {
        let mut command = Command::new(&cmd);
        command.args(args);
        command.current_dir(&dir);
        if let Ok(status) = command.status() {
            if let Some(exit_code) = status.code() {
                if exit_code == 0 {
                    folders.push(dir);
                }
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
