use crate::error::UserError;
use crate::helpers::get_subdirs;
use crate::operations;
use crate::runtime::Step;
use camino::Utf8PathBuf;

pub fn run(cmd: &str, args: &[String], current_dir: Utf8PathBuf) -> Result<Vec<Step>, UserError> {
    let mut result = vec![];
    let dirs = get_subdirs(&current_dir)?;
    let mut count = 1;
    for dir in dirs {
        result.push(operations::chdir(count, dir));
        count += 1;
        result.push(operations::execute(count, cmd.to_string(), args.to_owned()));
        count += 1;
    }
    result.push(operations::chdir(count, current_dir.into_string()));
    Ok(result)
}
