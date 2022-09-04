use crate::config::Config;
use crate::helpers::github;
use crate::runtime::Step;

pub fn clone(org: &str, dir: String) -> Config {
    let mut steps = vec![];
    let repos = github::get_repos(org);
    let mut id = 1;
    for repo in repos {
        steps.push(Step::Run {
            id,
            cmd: "git".into(),
            args: vec!["clone".into(), repo.ssh_url],
        });
        id += 1;
        if let Some(parent) = repo.parent {
            steps.push(Step::Run {
                id,
                cmd: "git".into(),
                args: vec![
                    "remote".into(),
                    "add".into(),
                    "upstream".into(),
                    parent.ssh_url,
                ],
            });
            id += 1;
        }
    }
    Config {
        steps,
        folders: None,
        root_dir: Some(dir),
    }
}
