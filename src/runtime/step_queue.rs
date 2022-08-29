use super::Step;
use crate::error::UserError;
use camino::Utf8Path;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, ErrorKind};
use std::path::PathBuf;

const FILENAME: &str = "mrt.json";

pub fn delete() -> Result<(), UserError> {
    let filepath = match location() {
        Some(path) => path,
        None => return Ok(()),
    };
    match fs::remove_file(filepath) {
        Ok(_) => Ok(()),
        Err(err) => Err(UserError::CannotDeletePersistenceFile {
            filename: FILENAME.into(),
            guidance: err.to_string(),
        }),
    }
}

/// provides the location of the persistence file
pub fn location() -> Option<PathBuf> {
    let path = PathBuf::from(".").join(FILENAME);
    if path.exists() {
        return Some(path);
    }
    let path = PathBuf::from("..").join(FILENAME);
    if path.exists() {
        return Some(path);
    }
    None
}

pub fn load() -> Result<Vec<Step>, UserError> {
    let filepath = match location() {
        Some(path) => path,
        None => return Ok(vec![]),
    };
    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return Ok(vec![]),
            _ => {
                return Err(UserError::CannotReadPersistenceFile {
                    guidance: e.to_string(),
                    filename: FILENAME.into(),
                })
            }
        },
    };
    let reader = BufReader::new(file);
    match serde_json::from_reader(reader) {
        Ok(result) => Ok(result),
        Err(err) => Err(UserError::InvalidPersistenceFormat {
            filename: FILENAME.into(),
            guidance: err.to_string(),
        }),
    }
}

pub fn save(dir: &Utf8Path, steps: &Vec<Step>) -> Result<(), UserError> {
    let filepath = dir.join(FILENAME);
    let file = File::create(&filepath).map_err(|err| UserError::CannotWriteFile {
        filename: filepath.to_string(),
        guidance: err.to_string(),
    })?;
    let writer = BufWriter::new(file);
    match serde_json::to_writer_pretty(writer, steps) {
        Ok(_) => Ok(()),
        Err(e) => Err(UserError::CannotWriteFile {
            filename: filepath.into(),
            guidance: e.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::runtime::step_queue;
    use crate::runtime::step_queue::FILENAME;
    use crate::runtime::Step;
    use camino::Utf8PathBuf;
    use std::fs;

    #[test]
    fn persistence() {
        let steps1 = vec![
            Step::Chdir {
                id: 2,
                dir: "abc".into(),
            },
            Step::Run {
                id: 3,
                cmd: "git".into(),
                args: vec!["clone".into()],
            },
        ];
        step_queue::save(&Utf8PathBuf::from("."), &steps1).unwrap();
        let steps2 = step_queue::load().unwrap();
        assert_eq!(steps1, steps2);
        fs::remove_file(FILENAME).unwrap();
    }
}
