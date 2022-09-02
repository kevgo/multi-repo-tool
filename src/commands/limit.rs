use crate::config::Config;
use crate::error::UserError;
use crate::helpers::get_subdirs;
use camino::Utf8Path;
use std::process::Command;

pub fn limit(cmd: &str, args: &[String], root_dir: &Utf8Path) -> Result<Config, UserError> {
    let mut folders = vec![];
    for dir in get_subdirs(root_dir)? {
        let mut command = Command::new(&cmd);
        command.args(args);
        if let Ok(status) = command.status() {
            if let Some(exit_code) = status.code() {
                if exit_code == 0 {
                    folders.push(dir);
                }
            }
        }
    }
    Ok(Config {
        folders: Some(folders),
        ..Config::default()
    })
}
