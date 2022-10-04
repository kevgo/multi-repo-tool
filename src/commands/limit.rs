use crate::config::Config;
use crate::error::UserError;
use crate::helpers::subdirs;
use camino::Utf8Path;
use colored::Colorize;
use std::process::{Command, ExitCode};

/// defines which folders get included
pub enum Mode {
    /// include folders that match the given condition
    Match,
    /// include folders that don't match the given condition
    NoMatch,
}

pub fn all(config: Config) -> (Config, Option<ExitCode>) {
    (
        Config {
            folders: None,
            ..config
        },
        None,
    )
}

pub fn only(
    cmd: &str,
    args: &[String],
    root_dir: &Utf8Path,
    mode: &Mode,
    config: Config,
) -> Result<(Config, Option<ExitCode>), UserError> {
    let mut new_folders = vec![];
    let all_folders = subdirs::all(root_dir)?;
    let all_folders_count = all_folders.len();
    let previous_count = config.folders.as_ref().map(|folders| folders.len());
    for dir in config.folders.unwrap_or(all_folders) {
        let mut command = Command::new(&cmd);
        command.args(args);
        command.current_dir(&dir);
        if let Ok(status) = command.status() {
            let folder_matches = match mode {
                Mode::Match => status.success(),
                Mode::NoMatch => !status.success(),
            };
            if folder_matches {
                new_folders.push(dir);
            }
        }
    }
    if new_folders.is_empty() {
        return Err(UserError::NoFoldersToIterate);
    }
    let output = match previous_count {
        Some(previous_count) => format!(
            "Tightening the existing limit of {} folders to {}/{} folders:",
            previous_count,
            new_folders.len(),
            all_folders_count
        ),
        None => format!(
            "Limiting execution to {}/{} folders:",
            new_folders.len(),
            all_folders_count
        ),
    };
    println!("\n{}", output.bold());
    for (i, folder) in new_folders.iter().enumerate() {
        println!("{}. {}", i + 1, folder);
    }
    Ok((
        Config {
            folders: Some(new_folders),
            ..Config::default()
        },
        None,
    ))
}
