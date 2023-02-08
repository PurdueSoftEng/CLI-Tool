extern crate octocrab;
use octocrab::{Octocrab, Page, Result, models, params};
use std::error::Error;

#[allow(non_snake_case)]
pub fn init_octo(token: String) -> Result<Octocrab, octocrab::Error>
{
    (Octocrab::builder().personal_token(token).build())
}

#[allow(non_snake_case)]
pub async fn get_repo(token: String, owner: String, repo: String) -> Result<octocrab::models::Repository, ()> {
    let octo = Octocrab::builder().personal_token(token).build().unwrap();
    let repo = match octo.repos(owner, repo).get().await {
        Ok(repo) => repo,
        Err(_) => return Err(()),
    };
    Ok(repo)
}

#[allow(non_snake_case)]
pub async fn get_issue(token: String, owner: String, repo: String) -> Result<Page<octocrab::models::issues::Issue>, octocrab::Error> {
    let route = format!("repos/{owner}/{repo}/issues/");
    let octo = Octocrab::builder().personal_token(token).build().unwrap();
    let mut page = octo.issues(owner, repo).list().state(params::State::All).send().await?;

    let mut i = 0;
    loop {
        for issue in &page.items {
            //println!("{}: {} -- {}", i, issue.title, issue.created_at);
            i += 1;
        }
        page = match octo
            .get_page::<models::issues::Issue>(&page.next)
            .await
        {
            Ok(Some(next_page)) => next_page,
            Ok(None) => break,
            Err(_) => break,
        }
    }

    Ok(page)
}

#[allow(non_snake_case)]
pub async fn get_issues(token: String, owner: String, repo: String, t: i64) -> Result<Vec<Vec<octocrab::models::issues::Issue>>> {
    let route = format!("repos/{owner}/{repo}/issues/");
    let octo = Octocrab::builder().personal_token(token).build().unwrap();
    
    let mut page = octo.issues(owner, repo).list().state(params::State::All).send().await?;
    let mut i = 0;
    let bin_size = chrono::Duration::days(t);
    let mut issues = Vec::new();
    let mut all_issues = Vec::new();

    loop {
        for issue in &page.items {

            let created_at = issue.created_at.naive_utc();
            let bin_start = created_at.date().and_hms(0, 0, 0);
            let bin_end = bin_start + bin_size;
            if created_at >= bin_start && created_at < bin_end {
                issues.push(issue.clone());
            }
            if created_at >= bin_end {
                all_issues.push(issues.clone());
                issues = Vec::new();
            }
        }

        if !issues.is_empty() {
            all_issues.push(issues.clone());
        }
        
        page = match octo
            .get_page::<models::issues::Issue>(&page.next)
            .await
        {
            Ok(Some(next_page)) => next_page,
            Ok(None) => break,
            Err(_) => break,
        }
    }

    Ok(all_issues)
}

#[allow(non_snake_case)]
pub fn get_avg_issue_duration(binnedIssues:Vec<Vec<octocrab::models::issues::Issue>>) -> f64{
    let mut durations: Vec<i64> = Vec::new();

    for bin in binnedIssues{
        println!("bin");
        for iss in bin{
            let issue_start = iss.created_at;
            if let Some(issue_end) = iss.closed_at {
                let duration = issue_end - issue_start;
                let duration_in_minutes = duration.num_seconds() / 60;
                println!("Duration in minutes: {}", duration_in_minutes);
                durations.push(duration_in_minutes);
            } 
        }
    }

    let sum: i64 = durations.iter().sum();
    let average = sum as f64 / durations.len() as f64;
    return average;
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
