use crate::github;
use crate::runtime::Step;

pub fn clone(org: &str) -> Vec<Step> {
    let mut result = vec![];
    let repos = github::get_repos(org);
    for (i, repo) in repos.into_iter().enumerate() {
        result.push(Step::Run {
            id: (i as u32) + 1,
            cmd: "git".into(),
            args: vec!["clone".into(), repo.clone_url],
        });
    }
    result
}
