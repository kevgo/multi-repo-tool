use super::Operation;
use std::process::Command;

#[derive(Debug, PartialEq)]
pub struct CloneOperation {
    repo_name: String,
    clone_url: String,
    repo_number: usize,
    repo_count: usize,
}

impl Operation for CloneOperation {
    fn execute(&self) {
        println!(
            "cloning repo {}/{}: {}",
            self.repo_number + 1,
            self.repo_count,
            self.repo_name
        );
        match Command::new("git")
            .args(["clone", &self.clone_url])
            .status()
        {
            Ok(status) => {}
            Err(err) => {}
        }
    }
}
