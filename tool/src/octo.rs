extern crate octocrab;
use octocrab::{Octocrab, Page, Result, models};

#[allow(non_snake_case)]
pub fn initOcto(token: String) -> Result<Octocrab, octocrab::Error>
{
    (Octocrab::builder().personal_token(token).build())
}

#[allow(non_snake_case)]
pub async fn getRepo(token: String, owner: String, repo: String) -> octocrab::models::Repository
{
    let octo = Octocrab::builder().personal_token(token).build().unwrap();
    match octo.repos(owner, repo).get().await
    {
        Ok(repo) => repo,
        Err(_) => panic!("Error fetching repo"),
    }
}

#[allow(non_snake_case)]
pub async fn getIssue(token: String, owner: String, repo: String) -> Page<octocrab::models::issues::Issue>
{
    let route = format!("repos/{owner}/{repo}/issues/");
    let octo = Octocrab::builder().personal_token(token).build().unwrap();

    match octo.issues(owner, repo).list().send().await
    {
        Ok(page) => page,
        Err(_) => panic!("Error fetching issue"),
    }
}
