use crate::config::Config;
use crate::error::UserError;
use crate::helpers::{folder_list, subdirs};
use camino::Utf8Path;
use colored::Colorize;
use std::io::{stdout, Write};
use std::process::{Command, ExitCode};
use walkdir::WalkDir;

/// defines which folders get included
pub enum Mode {
    /// include folders that match the given condition
    Match,
    /// include folders that don't match the given condition
    NoMatch,
}

const IGNORE: &[&str] = vec!["node_modules", "vendor"];

pub fn all(config: Config) -> (Config, Option<ExitCode>) {
    (
        Config {
            folders: None,
            steps: vec![],
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
    let previous_count = config.folders.as_ref().map(Vec::len);
    for dir in config.folders.unwrap_or(all_folders) {
        print!(".");
        let _ignore = stdout().flush();
        let mut command = Command::new(cmd);
        command.args(args);
        command.current_dir(&dir);
        if let Ok(output) = command.output() {
            let folder_matches = match mode {
                Mode::Match => output.status.success(),
                Mode::NoMatch => !output.status.success(),
            };
            if folder_matches {
                new_folders.push(dir);
            }
        }
    }
    println!("\n");
    if new_folders.is_empty() {
        return Err(UserError::NoFoldersToIterate);
    }
    let text = if let Some(previous_count) = previous_count {
        format!(
            "Tightening the existing limit of {}/{} folders further to {}/{} folders:",
            previous_count,
            all_folders_count,
            new_folders.len(),
            all_folders_count
        )
    } else {
        format!(
            "Limiting execution to {}/{} folders:",
            new_folders.len(),
            all_folders_count
        )
    };
    println!("{}", text.bold());
    println!("{}", folder_list::render(&new_folders));
    if !config.steps.is_empty() {
        println!("Discarding pending {} steps.", config.steps.len());
    }
    Ok((
        Config {
            folders: Some(new_folders),
            steps: vec![],
            ..config
        },
        None,
    ))
}

pub fn unfold(
    cmd: &str,
    args: &[String],
    root_dir: &Utf8Path,
    config: Config,
) -> Result<(Config, Option<ExitCode>), UserError> {
    let previous_count = config.folders.as_ref().map(Vec::len);
    let mut new_folders = vec![];
    let folders = match config.folders {
        Some(existing) => existing,
        None => subdirs::all(root_dir)?,
    };
    let folders_count = folders.len();
    for folder in folders {
        for entry in WalkDir::new(&folder) {
            let entry = entry.map_err(|err| UserError::CannotReadDirectory {
                directory: folder.clone(),
                guidance: err.to_string(),
            })?;
            if !entry.file_type().is_dir() {
                continue;
            }
            if IGNORE
                .iter()
                .any(|ignore| ignore == &entry.file_name().to_string_lossy())
            {
                continue;
            }
            print!(".");
            let _ignore = stdout().flush();
            let mut command = Command::new(cmd);
            command.args(args);
            command.current_dir(&folder);
            if let Ok(output) = command.output() {
                if output.status.success() {
                    new_folders.push(folder);
                }
            }
        }
    }
    println!("\n");
    if new_folders.is_empty() {
        return Err(UserError::NoFoldersToIterate);
    }
    let text = if let Some(previous_count) = previous_count {
        format!(
            "Tightening the existing limit of {}/{} folders further to {}/{} folders:",
            previous_count,
            folders_count,
            new_folders.len(),
            folders_count
        )
    } else {
        format!(
            "Limiting execution to {}/{} folders:",
            new_folders.len(),
            folders_count
        )
    };
    println!("{}", text.bold());
    println!("{}", folder_list::render(&new_folders));
    if !config.steps.is_empty() {
        println!("Discarding pending {} steps.", config.steps.len());
    }
    Ok((
        Config {
            folders: Some(new_folders),
            steps: vec![],
            ..config
        },
        None,
    ))
}
