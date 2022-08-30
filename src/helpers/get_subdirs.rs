use crate::error::UserError;
use camino::Utf8Path;
use std::fs::{self};

pub fn get_subdirs(path: &Utf8Path) -> Result<Vec<String>, UserError> {
    let entries = fs::read_dir(path).map_err(|err| UserError::CannotReadDirectory {
        directory: path.to_string(),
        guidance: err.to_string(),
    })?;
    let mut dirpaths: Vec<String> = entries
        .map(|entry| entry.expect("cannot read filesystem"))
        .filter(|entry| entry.metadata().expect("cannot read metadata").is_dir())
        .map(|entry| entry.path().to_string_lossy().to_string())
        .collect();
    dirpaths.sort();
    Ok(dirpaths)
}
