use crate::github;
use crate::operations;
use crate::runtime::Step;

pub fn clone(org: &str) -> Vec<Step> {
    let mut result = vec![];
    let repos = github::get_repos(org);
    for (i, repo) in repos.into_iter().enumerate() {
        result.push(operations::clone(i, repo.clone_url));
    }
    result
}
