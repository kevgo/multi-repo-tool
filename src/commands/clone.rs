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
    print!("fetch repos ...");
    let url = format!("https://api.github.com/orgs/{}/repos", org);
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();
    let response = client.get(url).send().unwrap();
    println!(" ok");
    let headers = next_page_url(response.headers());
    let repos = response.json::<Vec<GithubRepo>>().unwrap();
    // println!("{:#?}", resp);

    // clone each repo
    println!("cloning {} repos", repos.len());
    for (i, repo) in repos.iter().enumerate() {
        println!("cloning repo {}/{}: {}", i, repos.len(), repo.name);
    }
}

fn next_page_url(headers: &HeaderMap) -> String {
    for (name, value) in headers {
        if name != "link" {
            continue;
        }
        let s = value.to_str().unwrap();
        println!("value: {}", s);
    }
    String::new()
}

fn extract_next_link(value: &str) -> String {
    let re = Regex::new(r#"<([^>]+)>; rel="next""#).unwrap();
    let captures = re.captures(value).unwrap();
    let next = &captures[1];
    next.to_string()
}

#[cfg(test)]
mod tests {

    #[test]
    fn extract_next_link() {
        let give = r#"<https://api.github.com/organizations/108299804/repos?page=2>; rel="next", <https://api.github.com/organizations/108299804/repos?page=3>; rel="last""#;
        let want = "https://api.github.com/organizations/108299804/repos?page=2";
        let have = super::extract_next_link(give);
        assert_eq!(have, want)
    }
}
