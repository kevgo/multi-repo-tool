use crate::config::Config;
use crate::error::UserError;
use crate::helpers::github;
use crate::runtime::steps::{self, Step};

pub fn clone(org: &str, dir: String) -> Result<Config, UserError> {
    let mut steps = vec![];
    let repos = github::get_repos(org)?;
    for repo in repos {
        steps.push(Step::Run {
            cmd: "git".into(),
            args: vec!["clone".into(), repo.ssh_url],
        });
    }
    Ok(Config {
        steps: steps::numbered(steps),
        folders: None,
        root_dir: Some(dir),
    })
}
