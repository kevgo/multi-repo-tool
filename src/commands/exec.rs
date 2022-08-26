use crate::error::UserError;
use crate::operations;
use crate::runtime::Step;
use std::path::Path;
use std::{env, fs};

pub fn exec(cmd: &str, args: &[String]) -> Result<Vec<Step>, UserError> {
    let mut result = vec![];
    let current_dir = env::current_dir().expect("cannot determine current directory");
    let dirs = get_subdirs(&current_dir)?;
    for (i, dir) in dirs.into_iter().enumerate() {
        result.push(operations::chdir(i, dir));
        result.extend(operations::execute(i, cmd.to_string(), args.to_owned()));
    }
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
        if entry.metadata().unwrap().is_dir() {
            let dirname = entry.file_name().to_string_lossy().to_string();
            let dirpath = path.join(dirname);
            subdirs.push(dirpath.to_string_lossy().to_string());
        }
    }
    Ok(subdirs)
}
