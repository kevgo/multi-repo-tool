use super::Step;
use crate::error::UserError;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, ErrorKind};

const FILENAME: &str = "mrt.json";

/// removes the persistent task queue
pub fn forget() -> Result<(), UserError> {
    match fs::remove_file(FILENAME) {
        Ok(_) => Ok(()),
        Err(err) => Err(UserError::CannotDeletePersistenceFile {
            filename: FILENAME.into(),
            guidance: err.to_string(),
        }),
    }
}

/// loads an Executor instance from the persistence file on disk
pub fn load() -> Result<Vec<Step>, UserError> {
    let file = match File::open(FILENAME) {
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

/// stores the task queue on disk
pub fn persist(steps: &Vec<Step>) -> Result<(), UserError> {
    let file = match File::create(FILENAME) {
        Ok(file) => file,
        Err(e) => {
            return Err(UserError::CannotWritePersistenceFile {
                filename: FILENAME.into(),
                guidance: e.to_string(),
            })
        }
    };
    let writer = BufWriter::new(file);
    match serde_json::to_writer_pretty(writer, steps) {
        Ok(_) => Ok(()),
        Err(e) => Err(UserError::CannotWritePersistenceFile {
            filename: FILENAME.into(),
            guidance: e.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {

    use crate::runtime::persistence::FILENAME;
    use crate::runtime::{load, persist, Step};
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
                command: "git".into(),
                args: vec!["clone".into()],
            },
        ];
        persist(&steps1).unwrap();
        let steps2 = load().unwrap();
        assert_eq!(steps1, steps2);
        fs::remove_file(FILENAME).unwrap();
    }
}
