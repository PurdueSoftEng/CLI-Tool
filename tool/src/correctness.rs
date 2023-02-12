
use octocrab::{tokio::net::windows::named_pipe::PipeEnd::Client;
, Error};

// all issues closed (1/3 weight)
async fn are_all_issues_closed(repo: &str, client: &Client) -> Result<i32, Error> {
    let issues_url = format!("/repos/{}/issues", repo);
    let issues_response = client.get(issues_url).send().await?;

    let issues = issues_response.json::<Vec<serde_json::Value>>().await?;

    for issue in issues {
        let state = issue["state"].as_str().unwrap();
        if state != "closed" {
            return Ok(0);
        }
    }

    Ok(1)
}

// has tests (1/3 weight)
async fn has_testing_suite(repo: &str, client: &Client) -> Result<i32, Error> {
    let contents_url = format!("/repos/{}/contents", repo);
    let contents_response = client.get(contents_url).send().await?;

    let contents = contents_response.json::<Vec<serde_json::Value>>().await?;

    for content in contents {
        let content_name = content["name"].as_str().unwrap();
        if content_name == "tests" {
            return Ok(1);
        }
    }

    Ok(0)
}

// check number of releases (1/3 weight)
async fn check_number_of_releases(repo: &str, client: &Client) -> Result<i32, Error> {
    let releases_url = format!("/repos/{}/releases", repo);
    let releases_response = client.get(releases_url).send().await?;

    let releases = releases_response.json::<Vec<serde_json::Value>>().await?;

    if releases.len() > 10 {
        Ok(1)
    } else {
        Ok(0)
    }
}

async fn get_weighted_score(repo: &str, client: &Client) -> Result<f64, Error> {
    let first = are_all_issues_closed(repo, client).await?;
    let second = has_testing_suite(repo, client).await?;
    let third = check_number_of_releases(repo, client).await?;

    let total_score = (first as f64 / 3.0) + (second as f64 / 3.0) + (third as f64 / 3.0);

    Ok(total_score)
}