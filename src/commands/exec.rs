use crate::error::UserError;
use crate::operations;
use crate::runtime::Step;
use std::fs;
use std::path::Path;

pub fn exec(cmd: &str, args: &[String], current_dir: &Path) -> Result<Vec<Step>, UserError> {
    let mut result = vec![];
    let dirs = get_subdirs(&current_dir)?;
    let mut count = 1;
    for dir in dirs {
        result.push(operations::chdir(count, dir));
        count += 1;
        result.extend(operations::execute(count, cmd.to_string(), args.to_owned()));
        count += 1;
    }
    result.push(operations::chdir(count, current_dir));
    Ok(result)
}

fn get_subdirs(path: &Path) -> Result<Vec<String>, UserError> {
    let entries = match fs::read_dir(".") {
        Ok(entries) => entries,
        Err(err) => {
            return Err(UserError::CannotReadDirectory {
                directory: path.to_string_lossy().to_string(),
                guidance: err.to_string(),
            })
        }
    };
    let mut subdirs: Vec<String> = vec![];
    for entry in entries {
        let entry = entry.expect("cannot read filesystem");
        if entry
            .metadata()
            .expect("cannot read filesystem entry")
            .is_dir()
        {
            let dirname = entry.file_name().to_string_lossy().to_string();
            let dirpath = path.join(dirname);
            subdirs.push(dirpath.to_string_lossy().to_string());
        }
    }
    Ok(subdirs)
}
