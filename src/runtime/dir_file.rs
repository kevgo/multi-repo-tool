use crate::error::UserError;
use std::fs;
use std::path::Path;

const FILENAME: &str = "mrt.nextdir";

pub fn save(root: &Path, next_dir: &str) -> Result<(), UserError> {
    let filepath = root.join(FILENAME);
    fs::write(&filepath, next_dir).map_err(|err| UserError::CannotWriteFile {
        filename: filepath.to_string_lossy().to_string(),
        guidance: err.to_string(),
    })?;
    Ok(())
}
