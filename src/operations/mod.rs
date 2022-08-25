mod clone_repo;

use core::fmt::Debug;

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    CloneRepo { name: String, url: String },
}

#[derive(Debug, PartialEq, Eq)]
struct Executor {
    /// the operations to execute
    operations: Vec<Operation>,
}

impl Executor {
    /// loads an Executor instance from the persistence file on disk
    fn load() -> Executor {}

    /// stores this Executor into the persistence file on disk
    fn save(self) {}

    fn run(&mut self) {}
}

#[cfg(test)]
mod tests {

    mod Executor {
        use crate::operations::{Executor, Operation};

        #[test]
        fn persistence() {
            let executor1 = Executor {
                operations: vec![Operation::CloneRepo {
                    name: "test-repo".into(),
                    url: "git@github.com/test-org/test-repo".into(),
                    number: 3,
                    repo_count: 10,
                }],
            };
            executor1.save();
            let executor2 = Executor::load();
            assert_eq!(executor1, executor2);
        }
    }
}
