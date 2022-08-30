use crate::error::UserError;
use camino::Utf8PathBuf;
use std::fs;

const FILENAME: &str = "mrt.nextdir";

pub fn save(initial_dir: Utf8PathBuf, next_dir: &str) -> Result<(), UserError> {
    let filepath = initial_dir.join(FILENAME);
    fs::write(&filepath, next_dir).map_err(|err| UserError::CannotWriteFile {
        filename: filepath.to_string(),
        guidance: err.to_string(),
    })?;
    Ok(())
}
