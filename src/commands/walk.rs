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
    let full_start = start.map(|start| root_dir.join(start));
    let mut steps = vec![];
    let mut push = full_start.is_none();
    let dirs = match config.folders.clone() {
        None => get_subdirs(root_dir)?,
        Some(folders) => folders,
    };
    for dir in dirs {
        if let Some(start) = &full_start {
            println!("dir: {}, start: {}", dir, start);
            if &dir == start {
                push = true;
            }
        }
        if push {
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
