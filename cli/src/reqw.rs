use anyhow::Context;
use reqwest::header;

pub fn client(token: &str) -> anyhow::Result<reqwest::Client> {
    let mut headers = header::HeaderMap::new();
    let t = format!("Bearer {}", std::env::var("GITHUB_TOKEN")?);
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&t)?);
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/vnd.github.hawkgirl-preview+json"),
    );
    headers.insert(
        "X-GitHub-Api-Version",
        header::HeaderValue::from_static("2022-11-28"),
    );
    reqwest::Client::builder()
        .user_agent("ECE461_Team19_CLI")
        .default_headers(headers)
        .build()
        .context("failed to create http client to calculate metrics")
}

// GitHub GraphQL API
async fn graphql(client: &reqwest::Client, query: String) -> reqwest::Result<reqwest::Response> {
    client
        .post("https://api.github.com/graphql")
        .bearer_auth(format!("{}", std::env::var("GITHUB_TOKEN").unwrap()))
        .body(query)
        .send()
        .await
}

// GraphQL API call in json format
pub async fn graph_json(
    client: &reqwest::Client,
    query: String,
) -> reqwest::Result<serde_json::Value> {
    graphql(client, query)
        .await?
        .json::<serde_json::Value>()
        .await
}
