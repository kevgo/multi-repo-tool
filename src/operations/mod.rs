mod clone_repo;

use core::fmt::Debug;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};

const FILENAME: &str = "mrt.json";

/// the different operations that this tool can do
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
enum Operation {
    CloneRepo { name: String, url: String },
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
struct ExecutionStep {
    operation: Operation,
    repo_number: usize,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
struct Executor {
    /// the operations to execute
    steps: Vec<ExecutionStep>,
    repo_count: usize,
}

impl Executor {
    /// loads an Executor instance from the persistence file on disk
    fn load() -> Result<Option<Executor>, Box<dyn Error>> {
        let file = match File::open(FILENAME) {
            Ok(file) => file,
            Err(_) => return Ok(None),
        };
        let reader = BufReader::new(file);
        let executor: Executor = serde_json::from_reader(reader)?;
        Ok(Some(executor))
    }

    /// stores this Executor into the persistence file on disk
    fn save(&self) -> Result<(), Box<dyn Error>> {
        let file = File::create(FILENAME)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }

    fn run(&mut self) {}
}

#[cfg(test)]
mod tests {

    mod Executor {
        use crate::operations::{ExecutionStep, Executor, Operation};

        #[test]
        fn persistence() {
            let executor1 = Executor {
                steps: vec![ExecutionStep {
                    operation: Operation::CloneRepo {
                        name: "test-repo".into(),
                        url: "git@github.com/test-org/test-repo".into(),
                    },
                    repo_number: 3,
                }],
                repo_count: 10,
            };
            executor1.save();
            let executor2 = Executor::load().unwrap().unwrap();
            assert_eq!(executor1, executor2);
        }
    }
}
