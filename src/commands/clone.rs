use crate::github;

pub fn clone(org: &str) {
    println!("Cloning {} ...", org);
    let repos = github::get_repos(org);

    // clone each repo
    println!("cloning {} repos", repos.len());
    for (i, repo) in repos.iter().enumerate() {
        github::clone_repo(repo, i, repos.len());
    }
}
