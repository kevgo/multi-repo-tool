mod clone_repo;
mod fetch_org_repos;

pub use clone_repo::clone_repo;
pub use fetch_org_repos::get_repos;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GithubRepo {
    pub name: String,
    pub clone_url: String,
}
