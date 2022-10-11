use crate::config::Config;
use crate::error::UserError;
use crate::helpers::{folder_list, subdirs};
use camino::Utf8Path;
use colored::Colorize;
use std::io::{stdout, Write};
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
        let mut command = Command::new(&cmd);
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
