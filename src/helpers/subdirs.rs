use crate::error::UserError;
use camino::Utf8Path;
use std::fs::{self};

/// provides the name of all directories in the given path.
pub fn all(path: &Utf8Path) -> Result<Vec<String>, UserError> {
  let entries = fs::read_dir(path).map_err(|err| UserError::CannotReadDirectory {
    directory: path.to_string(),
    guidance: err.to_string(),
  })?;
  let mut dirpaths: Vec<String> = entries
    .map(|entry| entry.expect("cannot read filesystem"))
    .filter(|entry| entry.metadata().expect("cannot read metadata").is_dir())
    .map(|dir| dir.path().to_string_lossy().to_string())
    .collect();
  dirpaths.sort();
  Ok(dirpaths)
}

/// provides the number of all directories in the given path
pub fn count<AP: AsRef<Utf8Path>>(path: AP) -> Result<usize, UserError> {
  let path = path.as_ref();
  let entries = fs::read_dir(path).map_err(|err| UserError::CannotReadDirectory {
    directory: path.to_string(),
    guidance: err.to_string(),
  })?;
  Ok(
    entries
      .map(|entry| entry.expect("cannot read filesystem"))
      .filter(|entry| entry.metadata().expect("cannot read metadata").is_dir())
      .count(),
  )
}
