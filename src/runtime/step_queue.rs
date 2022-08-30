use super::Step;
use crate::error::UserError;
use camino::{Utf8Path, Utf8PathBuf};
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, ErrorKind};

pub const FILENAME: &str = "mrt.json";

pub fn delete(config_path: &Utf8Path) -> Result<(), UserError> {
    match fs::remove_file(config_path) {
        Ok(_) => Ok(()),
        Err(err) => Err(UserError::CannotDeletePersistenceFile {
            filename: FILENAME.into(),
            guidance: err.to_string(),
        }),
    }
}

/// provides the location of the persistence file
pub fn location(initial_dir: &Utf8Path) -> Option<Utf8PathBuf> {
    let path = initial_dir.join(FILENAME);
    if path.exists() {
        return Some(path);
    }
    let parent = match initial_dir.parent() {
        Some(parent) => parent,
        None => return None,
    };
    let path = parent.join(FILENAME);
    if path.exists() {
        return Some(path);
    }
    None
}

pub fn load(config_path: &Utf8Path) -> Result<Vec<Step>, UserError> {
    let file = match File::open(config_path) {
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
    serde_json::from_reader(reader).map_err(|err| UserError::InvalidPersistenceFormat {
        filename: FILENAME.into(),
        guidance: err.to_string(),
    })
}

pub fn save(config_path: Utf8PathBuf, steps: &Vec<Step>) -> Result<(), UserError> {
    let file = File::create(&config_path).map_err(|err| UserError::CannotWriteFile {
        filename: config_path.to_string(),
        guidance: err.to_string(),
    })?;
    let writer = BufWriter::new(file);
    match serde_json::to_writer_pretty(writer, steps) {
        Ok(_) => Ok(()),
        Err(e) => Err(UserError::CannotWriteFile {
            filename: config_path.into(),
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
    use std::{env, fs};

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
        let current = env::current_dir().unwrap();
        let config_path = current.join(FILENAME);
        let config_path = Utf8PathBuf::from_path_buf(config_path).unwrap();
        step_queue::save(config_path.clone(), &steps1).unwrap();
        let steps2 = step_queue::load(&config_path).unwrap();
        assert_eq!(steps1, steps2);
        fs::remove_file(config_path).unwrap();
    }
}
