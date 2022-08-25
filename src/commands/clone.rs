use crate::github;
use crate::runtime::{Operation, Step};

pub fn clone(org: &str) -> Vec<Step> {
    let repos = github::get_repos(org);

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
