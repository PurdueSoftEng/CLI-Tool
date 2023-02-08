extern crate octocrab;
use octocrab::{Octocrab, Page, Result, models};
use serde_json;

pub fn init_octo(token: String) -> Result<Octocrab, octocrab::Error>
{
    (Octocrab::builder().personal_token(token).build())
}

pub async fn get_repo(token: String, owner: String, repo: String) -> octocrab::models::Repository
{
    let octo = Octocrab::builder().personal_token(token).build().unwrap();
    match octo.repos(owner, repo).get().await
    {
        Ok(repo) => repo,
        Err(_) => panic!("Error fetching repo"),
    }
}

pub async fn get_issues(token: String, owner: String, repo: String) -> Page<octocrab::models::issues::Issue>
{
    let octo = Octocrab::builder().personal_token(token).build().unwrap();

    match octo.issues(owner, repo).list().send().await
    {
        Ok(page) => page,
        Err(_) => panic!("Error fetching issue"),
    }
}

pub async fn get_pulls(token: String, owner: String, repo: String) ->Page<octocrab::models::pulls::PullRequest>
{
    let octo = Octocrab::builder().personal_token(token).build().unwrap();

    match octo.pulls(owner, repo).list().send().await
    {
        Ok(page) => page,
        Err(_) => panic!("Error fetching issue"),
    }
}

// Example of using GraphQL
pub async fn get_num_commits(token: String, owner: &str, repo: &str) -> serde_json::Value
{
    let v = vec!["query {repository(owner: \"", owner, "\", name: \"", repo, "\") {object(expression: \"master\") {... on Commit {history {totalCount}}}}}"];

    let str: String = v.concat();

    println!("{}", str);

    let octo = Octocrab::builder().personal_token(token).build().unwrap();

    match octo.graphql(&str).await
    {
        Ok(json) => json,
        Err(_) => panic!("Error with query"),
    }
}
