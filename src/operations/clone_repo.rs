use std::process::Command;

pub fn print(name: usize, number: usize, count: usize) {
    println!("cloning repo {}/{}: {}", number + 1, count, name);
}

pub fn clone_repo(clone_url: &str) {
    match Command::new("git").args(["clone", clone_url]).status() {
        Ok(status) => {}
        Err(err) => {}
    }
}
