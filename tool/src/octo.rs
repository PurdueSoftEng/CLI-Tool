extern crate octocrab;
use octocrab::{Octocrab, Page, Result, models};

pub fn initOcto(token: String) -> Result<Octocrab, octocrab::Error>
{
    (Octocrab::builder().personal_token(token).build())
}

pub async fn getRepo(token: String, owner: String, repo: String) -> octocrab::models::Repository
{
    let octo = Octocrab::builder().personal_token(token).build().unwrap();
    match octo.repos(owner, repo).get().await
    {
        Ok(repo) => repo,
        Err(_) => panic!("Error fetching repo"),
    }
}

pub async fn getIssues(token: String, owner: String, repo: String) -> Page<octocrab::models::issues::Issue>
{
    let octo = Octocrab::builder().personal_token(token).build().unwrap();

    match octo.issues(owner, repo).list().send().await
    {
        Ok(page) => page,
        Err(_) => panic!("Error fetching issue"),
    }
}

pub async fn getPulls(token: String, owner: String, repo: String) ->Page<octocrab::models::pulls::PullRequest>
{
    let octo = Octocrab::builder().personal_token(token).build().unwrap();
    
    match octo.pulls(owner, repo).list().send().await
    {
        Ok(page) => page,
        Err(_) => panic!("Error fetching issue"),
    }
}