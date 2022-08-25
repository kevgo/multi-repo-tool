use crate::github;
use crate::runtime::{Operation, Step};
use std::io::Write;

pub fn clone(org: &str) -> Vec<Step> {
    print!("fetching Github org {} ...", org);
    let _ = std::io::stdout().flush();
    let repos = github::get_repos(org);
    println!(" {} repos found", repos.len());

    // clone each repo
    let mut result = vec![];
    for (i, repo) in repos.into_iter().enumerate() {
        result.push(Step {
            operation: Operation::CloneRepo {
                name: repo.name,
                url: repo.clone_url,
            },
            step_number: i,
        });
    }
    result
}
