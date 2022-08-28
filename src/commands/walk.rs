use crate::error::UserError;
use crate::helpers::get_subdirs;
use crate::runtime::Step;
use camino::Utf8PathBuf;

pub fn walk(current_dir: Utf8PathBuf) -> Result<Vec<Step>, UserError> {
    let mut result = vec![];
    let dirs = get_subdirs(&current_dir)?;
    let mut count = 1;
    for dir in dirs {
        result.push(Step::Chdir { id: count, dir });
        count += 1;
        result.push(Step::Exit { id: count });
        count += 1;
    }
    result.push(Step::Chdir {
        id: count,
        dir: current_dir.into_string(),
    });
    Ok(result)
}
