use crate::error::UserError;
use crate::helpers::get_subdirs;
use crate::runtime::Step;
use camino::Utf8Path;

pub fn walk(root_dir: &Utf8Path) -> Result<Vec<Step>, UserError> {
    let mut result = vec![];
    let dirs = get_subdirs(root_dir)?;
    for (i, dir) in dirs.into_iter().enumerate() {
        result.push(Step::Chdir {
            id: (i as u32) + 1,
            dir,
        });
        result.push(Step::Exit { id: (i as u32) + 1 });
    }
    result.push(Step::Chdir {
        id: (result.len() as u32) + 1,
        dir: root_dir.to_string(),
    });
    Ok(result)
}
