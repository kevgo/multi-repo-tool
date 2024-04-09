use crate::error::UserError;
use camino::Utf8PathBuf;
use std::{env, fs};

/// provides the full path to the config file
pub fn filepath() -> Utf8PathBuf {
  let fullpath = format!(
    "{}/.config/mrt.next_dir",
    env::var("HOME").expect("cannot read environment variable $HOME")
  );
  Utf8PathBuf::from(fullpath)
}

pub fn save(next_dir: &str) -> Result<(), UserError> {
  fs::write(filepath(), next_dir).map_err(|err| UserError::CannotWriteFile {
    filename: filepath().to_string(),
    guidance: err.to_string(),
  })?;
  Ok(())
}
