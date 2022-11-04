use crate::config::Config;
use crate::error::UserError;
use crate::helpers::{folder_list, subdirs};
use camino::Utf8Path;
use colored::Colorize;
use std::io::{stdout, Write};
use std::process::{Command, ExitCode};

pub fn list(
    cmd: &str,
    args: &[String],
    root_dir: &Utf8Path,
    config: Config,
) -> Result<(Config, Option<ExitCode>), UserError> {
    let mut matching_folders = vec![];
    let all_folders = subdirs::all(root_dir)?;
    let folders_count = all_folders.len();
    for dir in config.folders.unwrap_or(all_folders) {
        print!(".");
        let _ignore = stdout().flush();
        let mut command = Command::new(cmd);
        command.args(args);
        command.current_dir(&dir);
        if let Ok(output) = command.output() {
            if output.status.success() {
                matching_folders.push(dir);
            }
        }
    }
    let init_len = root_dir.as_os_str().len() + 1;
    matching_folders = matching_folders
        .into_iter()
        .map(|folder| folder[init_len..].to_owned())
        .collect();
    println!(
        "\n\n{}",
        format!(
            "{}/{} folders match:",
            matching_folders.len(),
            folders_count
        )
        .bold()
    );
    println!("{}", folder_list::render(&matching_folders));
    Ok((Config::default(), Some(ExitCode::SUCCESS)))
}
