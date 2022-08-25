use std::process::Command;

pub fn print(name: usize, number: usize, count: usize) {
    println!("cloning repo {}/{}: {}", number + 1, count, name);
}

pub fn clone_repo(clone_url: &str) {
    let command = Command::new("git").args(["clone", clone_url]);
    match command.status() {
        Ok(status) => {}
        Err(err) => {}
    }
}
