use super::Step;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::mem::drop;

const FILENAME: &str = "mrt.json";

/// removes the persistent task queue
pub fn forget() {
    drop(fs::remove_file(FILENAME));
}

/// loads an Executor instance from the persistence file on disk
pub fn load() -> Result<Option<Vec<Step>>, Box<dyn Error>> {
    let file = match File::open(FILENAME) {
        Ok(file) => file,
        Err(_) => return Ok(None),
    };
    let reader = BufReader::new(file);
    let result: Vec<Step> = serde_json::from_reader(reader)?;
    Ok(Some(result))
}

/// stores the task queue on disk
pub fn persist(steps: &Vec<Step>) -> Result<(), Box<dyn Error>> {
    let file = File::create(FILENAME)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, steps)?;
    Ok(())
}

#[cfg(test)]
mod tests {

    mod persistence {
        use crate::runtime::persistence::FILENAME;
        use crate::runtime::{load, persist, Step};
        use std::fs;
        use std::mem::drop;

        #[test]
        fn persistence() {
            let steps1 = vec![Step {
                id: 3,
                command: "git".into(),
                args: vec!["clone".into()],
            }];
            drop(persist(&steps1));
            let steps2 = load().unwrap().unwrap();
            assert_eq!(steps1, steps2);
            drop(fs::remove_file(FILENAME));
        }
    }
}
