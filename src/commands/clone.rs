use crate::github;
use crate::runtime;
use crate::runtime::Step;

pub fn clone(org: &str) -> Vec<Step> {
    let repos = github::get_repos(org);

    let mut result = vec![];
    for (i, repo) in repos.into_iter().enumerate() {
        result.push(runtime::clone(i, repo.clone_url));
    }
    result
}
