use crate::config::Config;
use crate::error::UserError;
use crate::helpers::get_subdirs;
use crate::runtime::steps::{self, Step};
use camino::Utf8Path;

pub fn walk(
    root_dir: &Utf8Path,
    config: Config,
    start: Option<String>,
) -> Result<Config, UserError> {
    let start = start.map(|start| root_dir.join(start));
    let mut steps = vec![];
    let mut active = start.is_none();
    let dirs = match config.folders.clone() {
        None => get_subdirs(root_dir)?,
        Some(folders) => folders,
    };
    for dir in dirs {
        if let Some(start) = &start {
            if &dir == start {
                active = true;
            }
        }
        if active {
            steps.push(Step::Chdir { dir });
            steps.push(Step::Exit);
        }
    }
    steps.push(Step::Chdir {
        dir: root_dir.to_string(),
    });
    Ok(Config {
        steps: steps::numbered(steps),
        root_dir: Some(root_dir.to_string()),
        ..config
    })
}
