use crate::config::Config;
use crate::error::UserError;
use crate::helpers::github;
use crate::runtime::Step;

pub fn clone(org: &str, dir: String) -> Result<Config, UserError> {
    let mut steps = vec![];
    let repos = github::get_repos(org)?;
    for (i, repo) in repos.into_iter().enumerate() {
        steps.push(Step::Run {
            id: (i as u32) + 1,
            cmd: "git".into(),
            args: vec!["clone".into(), repo.ssh_url],
        });
    }
    Ok(Config {
        steps,
        folders: None,
        root_dir: Some(dir),
    })
}
