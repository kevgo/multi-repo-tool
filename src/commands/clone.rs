use regex::Regex;
use reqwest::header::HeaderMap;
use serde::Deserialize;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Deserialize)]
struct GithubRepo {
    name: String,
    clone_url: String,
}

pub fn clone(org: &str) {
    println!("Cloning... {}", org);
    // get repos
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();
    let mut repos: Vec<GithubRepo> = vec![];
    let mut url = Some(format!("https://api.github.com/orgs/{}/repos", org));
    while url.is_some() {
        print!("fetch {:?} ...", &url);
        let response = client.get(&url.unwrap()).send().unwrap();
        println!(" ok");
        url = next_page_url(response.headers());
        let parsed = response.json::<Vec<GithubRepo>>().unwrap();
        repos.extend(parsed);
    }

    // println!("{:#?}", resp);

    // clone each repo
    println!("cloning {} repos", repos.len());
    for (i, repo) in repos.iter().enumerate() {
        println!("cloning repo {}/{}: {}", i + 1, repos.len(), repo.name);
    }
}

fn next_page_url(headers: &HeaderMap) -> Option<String> {
    for (name, value) in headers {
        if name == "link" {
            return extract_next_link(value.to_str().unwrap());
        }
    }
    None
}

fn extract_next_link(value: &str) -> Option<String> {
    // TODO: cache the regex
    let re = Regex::new(r#"<([^>]+)>; rel="next""#).unwrap();
    match re.captures(value) {
        None => return None,
        Some(captures) => match captures.len() {
            0 => return None,
            1 => return None,
            2 => return Some(captures[1].to_string()),
            _ => panic!(""),
        },
    }
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
            assert_eq!(have, want)
        }
    }
}
