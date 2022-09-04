use crate::config::Config;
use crate::error::UserError;
use crate::helpers::get_subdirs;
use crate::runtime::steps::{self, Step};
use camino::Utf8PathBuf;

pub fn run(
    cmd: &str,
    args: &[String],
    config: Config,
    root_dir: &Utf8PathBuf,
) -> Result<Config, UserError> {
    let mut steps = vec![];
    let dirs = match config.folders.clone() {
        None => get_subdirs(root_dir)?,
        Some(folders) => folders,
    };
    for dir in dirs {
        steps.push(Step::Chdir { dir });
        steps.push(Step::Run {
            cmd: cmd.to_string(),
            args: args.to_owned(),
        });
    }
    Ok(Config {
        steps: steps::numbered(steps),
        root_dir: Some(root_dir.to_string()),
        ..config
    })
}
