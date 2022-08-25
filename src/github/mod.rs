mod fetch_org_repos;

pub use fetch_org_repos::get_repos;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Repo {
    pub name: String,
    pub clone_url: String,
}
