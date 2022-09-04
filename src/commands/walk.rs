use crate::config::Config;
use crate::error::UserError;
use crate::helpers::get_subdirs;
use crate::runtime::steps::{self, Step};
use camino::Utf8Path;

pub fn walk(root_dir: &Utf8Path, config: Config) -> Result<Config, UserError> {
    let mut steps = vec![];
    let dirs = get_subdirs(root_dir)?;
    for dir in dirs {
        steps.push(Step::Chdir { dir });
        steps.push(Step::Exit);
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
