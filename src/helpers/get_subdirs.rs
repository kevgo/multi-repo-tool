use crate::error::UserError;
use camino::Utf8Path;
use std::fs;

pub fn get_subdirs(path: &Utf8Path) -> Result<Vec<String>, UserError> {
    let entries = fs::read_dir(".").map_err(|err| UserError::CannotReadDirectory {
        directory: path.to_string(),
        guidance: err.to_string(),
    })?;
    let mut subdirs: Vec<String> = vec![];
    for entry in entries {
        let entry = entry.expect("cannot read filesystem");
        if entry
            .metadata()
            .expect("cannot read filesystem metadata")
            .is_dir()
        {
            let dirname = entry.file_name().to_string_lossy().to_string();
            let dirpath = path.join(dirname);
            subdirs.push(dirpath.to_string());
        }
    }
    Ok(subdirs)
}
