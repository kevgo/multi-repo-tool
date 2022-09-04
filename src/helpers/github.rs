use crate::error::UserError;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use serde::Deserialize;
use std::io;
use std::io::Write;
use std::mem::drop;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
static LINK_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<([^>]+)>; rel="next""#).unwrap());

#[derive(Deserialize)]
pub struct Repo {
    /// URL for API access
    pub url: String,
    /// URL for `git clone`
    pub ssh_url: String,
    /// the parent of the fork
    pub parent: Option<RepoParent>,
}

#[derive(Deserialize)]
pub struct RepoParent {
    pub ssh_url: String,
}

#[derive(Deserialize)]
pub struct ErrorMessage {
    message: String,
    documentation_url: String,
}

pub fn get_repos(org: &str) -> Result<Vec<Repo>, UserError> {
    print!("fetching Github org {} .", org);
    drop(io::stdout().flush());
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .expect("cannot build HTTP client");
    let mut org_repos: Vec<Repo> = vec![];
    let mut next_url = Some(format!("https://api.github.com/orgs/{}/repos", org));
    while let Some(url) = next_url {
        let response = client.get(&url).send().expect("HTTP request failed");
        print!(".");
        io::stdout().flush().expect("cannot flush stdout");
        next_url = next_page_url(response.headers());
        match response.status() {
            StatusCode::OK => {
                let parsed: Vec<Repo> = response.json().expect("cannot parse Github API response");
                org_repos.extend(parsed);
            }
            StatusCode::FORBIDDEN => {
                let error: ErrorMessage = response.json().expect("cannot parse Github API error");
                return Err(UserError::ApiRequestFailed {
                    url,
                    error: error.message,
                    guidance: error.documentation_url,
                });
            }
            code => {
                return Err(UserError::UnknownApiError {
                    url,
                    code: code.as_u16(),
                    response: response.text().expect("cannot convert API error to text"),
                })
            }
        }
    }
    let mut repos: Vec<Repo> = vec![];
    for org_repo in org_repos {
        let response = client
            .get(&org_repo.url)
            .send()
            .expect("HTTP request failed");
        print!(".");
        io::stdout().flush().expect("cannot flush stdout");
        let parsed: Repo = response.json().expect("cannot parse Github API response");
        repos.push(parsed);
    }
    println!(" {} repositories found", repos.len());
    Ok(repos)
}

/// provides the URL to the next set of paginated API results
fn next_page_url(headers: &HeaderMap) -> Option<String> {
    for (name, value) in headers {
        if name == "link" {
            return extract_next_link(value.to_str().unwrap());
        }
    }
    None
}

fn extract_next_link(value: &str) -> Option<String> {
    LINK_RE
        .captures(value)
        .map(|captures| captures[1].to_string())
}

#[cfg(test)]
mod tests {

    mod extract_next_link {

        #[test]
        fn matches() {
            let give = r#"<https://api.github.com/organizations/108299804/repos?page=2>; rel="next", <https://api.github.com/organizations/108299804/repos?page=3>; rel="last""#;
            let want =
                Some("https://api.github.com/organizations/108299804/repos?page=2".to_string());
            let have = super::super::extract_next_link(give);
            assert_eq!(have, want);
        }
        #[test]
        fn no_match() {
            let give = r#"<https://api.github.com/organizations/108299804/repos?page=2>; rel="prev", <https://api.github.com/organizations/108299804/repos?page=3>; rel="last""#;
            let want = None;
            let have = super::super::extract_next_link(give);
            assert_eq!(have, want);
        }
    }
}
