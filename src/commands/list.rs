use crate::config::Config;
use crate::error::UserError;
use crate::helpers::{folder_list, subdirs};
use camino::Utf8Path;
use colored::Colorize;
use std::process::{Command, ExitCode};

pub fn list(
    cmd: &str,
    args: &[String],
    root_dir: &Utf8Path,
    config: Config,
) -> Result<(Config, Option<ExitCode>), UserError> {
    let mut matching_folders = vec![];
    let all_folders = subdirs::all(root_dir)?;
    for dir in config.folders.unwrap_or(all_folders) {
        let mut command = Command::new(&cmd);
        command.args(args);
        command.current_dir(&dir);
        if let Ok(status) = command.status() {
            if status.success() {
                matching_folders.push(dir);
            }
        }
    }
    let init_len = root_dir.as_os_str().len() + 1;
    matching_folders = matching_folders
        .into_iter()
        .map(|folder| folder[init_len..].to_owned())
        .collect();
    println!("\n{}", "Successful folders:".bold());
    println!("{}", folder_list::render(&matching_folders));
    Ok((Config::default(), Some(ExitCode::SUCCESS)))
}
