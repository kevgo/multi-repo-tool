use super::Step;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};

const FILENAME: &str = "mrt.json";

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

/// stores this Executor into the persistence file on disk
pub fn save(steps: &Vec<Step>) -> Result<(), Box<dyn Error>> {
    let file = File::create(FILENAME)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, steps)?;
    Ok(())
}

pub fn run(steps: &Vec<Step>) {}

#[cfg(test)]
mod tests {

    mod Executor {
        use crate::runtime::{load, save, Operation, Step};

        #[test]
        fn persistence() {
            let steps1 = vec![Step {
                operation: Operation::CloneRepo {
                    name: "test-repo".into(),
                    url: "git@github.com/test-org/test-repo".into(),
                },
                step_number: 3,
            }];
            save(&steps1);
            let steps2 = load().unwrap().unwrap();
            assert_eq!(steps1, steps2);
        }
    }
}
