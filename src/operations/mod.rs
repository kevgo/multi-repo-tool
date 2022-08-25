mod clone_repo;

use clone_repo::CloneOperation;

trait Operation {
    fn execute(&self);
}

#[derive(Debug, PartialEq)]
struct Executor {
    /// the operations to execute
    operations: Vec<Box<dyn Operation>>,
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
        use crate::operations::clone_repo::CloneOperation;
        use crate::operations::Executor;

        #[test]
        fn persistence() {
            let executor1 = Executor {
                operations: vec![Box::new(CloneOperation {
                    repo_name: "test-repo".into(),
                    clone_url: "git@github.com/test-org/test-repo".into(),
                    repo_number: 3,
                    repo_count: 10,
                })],
            };
            executor1.save();
            let executor2 = Executor::load();
            assert_eq!(executor1, executor2);
        }
    }
}
