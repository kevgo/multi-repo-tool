use crate::error::UserError;
use camino::Utf8Path;
use std::fs;

const FILENAME: &str = "mrt.nextdir";

pub fn save(root: &Utf8Path, next_dir: &str) -> Result<(), UserError> {
    let filepath = root.join(FILENAME);
    fs::write(&filepath, next_dir).map_err(|err| UserError::CannotWriteFile {
        filename: filepath.to_string(),
        guidance: err.to_string(),
    })?;
    Ok(())
}
