use crate::config::Config;
use crate::error::UserError;
use crate::helpers::get_subdirs;
use crate::runtime::Step;
use camino::Utf8Path;

pub fn walk(root_dir: &Utf8Path, config: Config) -> Result<Config, UserError> {
    let mut steps = vec![];
    let dirs = get_subdirs(root_dir)?;
    for (i, dir) in dirs.into_iter().enumerate() {
        steps.push(Step::Chdir {
            id: (i as u32) + 1,
            dir,
        });
        steps.push(Step::Exit { id: (i as u32) + 1 });
    }
    steps.push(Step::Chdir {
        id: (steps.len() as u32) / 2 + 1,
        dir: root_dir.to_string(),
    });
    Ok(Config {
        steps,
        root_dir: Some(root_dir.to_string()),
        ..config
    })
}
