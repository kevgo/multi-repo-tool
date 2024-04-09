use crate::config::Config;
use crate::error::UserError;
use crate::helpers::{folder_list, subdirs};
use crate::runtime::steps::NumberedStep;
use camino::Utf8Path;
use colored::Colorize;
use std::io::{stdout, Write};
use std::process::{Command, ExitCode};
use walkdir::WalkDir;

/// defines which folders get included
#[derive(Clone, Copy, Eq, PartialEq)]
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
  mode: Mode,
  config: Config,
) -> Result<(Config, Option<ExitCode>), UserError> {
  let mut new_folders = vec![];
  let all_folders = subdirs::all(root_dir)?;
  let all_folders_count = all_folders.len();
  let previous_count = config.folders.as_ref().map(Vec::len);
  for dir in config.folders.unwrap_or(all_folders) {
    print_dot();
    if command_success(&dir, cmd, args) ^ (mode == Mode::NoMatch) {
      new_folders.push(dir);
    }
  }
  new_folders.sort_unstable();
  println!("\n");
  if new_folders.is_empty() {
    return Err(UserError::NoFoldersToIterate);
  }
  let folders_count = all_folders_count;
  let steps: &[NumberedStep] = &config.steps;
  let text = if let Some(previous_count) = previous_count {
    format!(
            "Tightening the existing limit of {}/{} top-level folders further to {}/{} top-level folders:",
            previous_count,
            folders_count,
            new_folders.len(),
            folders_count
        )
  } else {
    format!(
      "Limiting execution to {}/{} top-level folders:",
      new_folders.len(),
      folders_count
    )
  };
  println!("{}", text.bold());
  println!("{}", folder_list::render(&new_folders));
  if !steps.is_empty() {
    println!("Discarding pending {} steps.", steps.len());
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
  let had_previous_limit = config.folders.is_some();
  let all_folders = subdirs::all(root_dir)?;
  let all_folders_count = all_folders.len();
  let folders = match config.folders {
    Some(existing) => existing,
    None => all_folders,
  };
  let mut new_folders = vec![];
  if !had_previous_limit && command_success(root_dir.as_str(), cmd, args) {
    new_folders.push(root_dir.to_string());
  }
  for folder in folders {
    for entry in WalkDir::new(&folder) {
      let entry = entry.map_err(|err| UserError::CannotReadDirectory {
        directory: folder.clone(),
        guidance: err.to_string(),
      })?;
      if !entry.file_type().is_dir() {
        continue;
      }
      if should_ignore(&entry.file_name().to_string_lossy()) {
        continue;
      }
      print_dot();
      let entry_path = entry.path().to_string_lossy();
      if command_success(&entry_path, cmd, args) {
        new_folders.push(entry_path.to_string());
      }
    }
  }
  new_folders.sort_unstable();
  println!("\n");
  if new_folders.is_empty() {
    return Err(UserError::NoFoldersToIterate);
  }
  let steps: &[NumberedStep] = &config.steps;
  let text = if let Some(previous_count) = previous_count {
    format!(
      "Unfolding the existing limit of {}/{} top-level folders to {} subfolders:",
      previous_count,
      all_folders_count,
      new_folders.len()
    )
  } else {
    format!("Unfolding execution to {} subfolders:", new_folders.len())
  };
  println!("{}", text.bold());
  println!("{}", folder_list::render(&new_folders));
  if !steps.is_empty() {
    println!("Discarding pending {} steps.", steps.len());
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

/// indicates whether the given folder should be ignored
fn should_ignore(path: &str) -> bool {
  matches!(path, "node_modules" | "vendor")
}

/// prints progress indicator to the CLI
fn print_dot() {
  print!(".");
  let _ignore = stdout().flush();
}

/// runs the given command in the given folder and indicates success
fn command_success(folder: &str, cmd: &str, args: &[String]) -> bool {
  let mut command = Command::new(cmd);
  command.args(args);
  command.current_dir(folder);
  if let Ok(output) = command.output() {
    output.status.success()
  } else {
    false
  }
}
