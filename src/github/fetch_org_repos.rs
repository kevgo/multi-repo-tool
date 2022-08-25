use crate::github;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::header::HeaderMap;
use std::io;
use std::io::Write;
use std::mem::drop;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
static LINK_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<([^>]+)>; rel="next""#).unwrap());

pub fn get_repos(org: &str) -> Vec<github::Repo> {
    print!("fetching Github org {} .", org);
    drop(io::stdout().flush());
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .expect("cannot build HTTP client");
    let mut repos: Vec<github::Repo> = vec![];
    let mut next_url = Some(format!("https://api.github.com/orgs/{}/repos", org));
    while let Some(url) = next_url {
        let response = client.get(&url).send().expect("HTTP request failed");
        print!(".");
        drop(io::stdout().flush());
        next_url = next_page_url(response.headers());
        let parsed = response
            .json::<Vec<github::Repo>>()
            .expect("cannot parse Github API response");
        repos.extend(parsed);
    }
    println!(" {} repositories found", repos.len());
    repos
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
        fn has_next_link() {
            let give = r#"<https://api.github.com/organizations/108299804/repos?page=2>; rel="next", <https://api.github.com/organizations/108299804/repos?page=3>; rel="last""#;
            let want =
                Some("https://api.github.com/organizations/108299804/repos?page=2".to_string());
            let have = super::super::extract_next_link(give);
            assert_eq!(have, want);
        }
    }
}
