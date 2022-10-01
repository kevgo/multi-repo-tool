use crate::config::Config;
use crate::error::UserError;
use crate::helpers::github;
use crate::runtime::steps::{self, Step};
use camino::Utf8Path;
use std::process::ExitCode;

pub fn clone(org: &str, dir: &Utf8Path) -> Result<(Config, Option<ExitCode>), UserError> {
    let mut steps = vec![];
    let repos = github::get_repos(org)?;
    for repo in repos {
        steps.push(Step::Run {
            cmd: "git".into(),
            args: vec!["clone".into(), repo.ssh_url],
        });
        if let Some(parent) = repo.parent {
            steps.push(Step::Chdir {
                dir: dir.join(repo.name).into_string(),
            });
            steps.push(Step::Run {
                cmd: "git".into(),
                args: vec![
                    "remote".into(),
                    "add".into(),
                    "upstream".into(),
                    parent.ssh_url,
                ],
            });
            steps.push(Step::Chdir {
                dir: dir.to_string(),
            });
        }
    }
    Ok((
        Config {
            steps: steps::numbered(steps),
            folders: None,
            root_dir: Some(dir.to_string()),
        },
        None,
    ))
}
