use crate::reqw;
use regex::Regex;

pub async fn calc_version(client: &reqwest::Client, owner: &str, repo: &str) -> f32 {
    let resp = reqw::graph_json(client, format!("{{ \"query\": \"query {{ repository(owner:\\\"{}\\\", name:\\\"{}\\\") {{ dependencyGraphManifests {{ nodes {{ dependencies {{ nodes {{ requirements }} }} }} }} }} }}\" }}", owner, repo)).await.unwrap();

    let caret = Regex::new(r"^\^\s?0\.\d.*").unwrap();
    let equal = Regex::new(r"^=\s?(\d\.\d|[a-zA-Z]+).*").unwrap();
    let tilde = Regex::new(r"^~\s?\d\.\d.*").unwrap();
    let range = Regex::new(r"^>=?\s?(\d+)\.(\d+).*(<|<=)\s?(\d+).(\d+).*").unwrap();
    let mut total = 0;
    let mut pinned = 0;
    for mani in resp["data"]["repository"]["dependencyGraphManifests"]["nodes"]
        .as_array()
        .unwrap()
    {
        for req in mani["dependencies"]["nodes"].as_array().unwrap() {
            total += 1;
            let ver = req["requirements"].as_str().unwrap();
            if caret.is_match(ver) {
                pinned += 1;
            } else if equal.is_match(ver) {
                pinned += 1;
            } else if tilde.is_match(ver) {
                pinned += 1;
            } else if let Some(cap) = range.captures(ver) {
                if &cap[3] == "<" {
                    if cap[1] == cap[4]
                        && cap[5].parse::<i32>().unwrap() - cap[2].parse::<i32>().unwrap() == 1
                    {
                        pinned += 1;
                    }
                } else {
                    if cap[1] == cap[4] && cap[2] == cap[5] {
                        pinned += 1;
                    }
                }
            }
        }
    }

    if total == 0 {
        0.0
    } else {
        pinned as f32 / total as f32
    }
}
