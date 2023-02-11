extern crate octocrab;
use octocrab::{Octocrab, Page, Result, models::{self, repos::RepoCommit}, params, commits};
use octocrab::models::Repository;
use chrono::{Duration, Utc};
use serde::__private::de::InternallyTaggedUnitVisitor;

pub fn get_avg_issue_duration(binned_issues:Vec<Vec<octocrab::models::issues::Issue>>) -> f64{
    let mut durations: Vec<i64> = Vec::new();

    for bin in binned_issues{
        //println!("bin");
        for iss in bin{
            let issue_start = iss.created_at;
            if let Some(issue_end) = iss.closed_at {
                let duration = issue_end - issue_start;
                let duration_in_minutes = duration.num_seconds() / 60;
                //println!("Duration in minutes: {}", duration_in_minutes);
                durations.push(duration_in_minutes);
            } 
        }
    }

    let sum: i64 = durations.iter().sum();
    let average = sum as f64 / durations.len() as f64;
    return average;
}

// pub fn calc_responsive_maintainer_summation(binned_commits:Vec<Vec<RepoCommit>>, t:f64) -> f64{
//     let mut total:f64 = 0.0;
//     for bin in binned_commits{
//         println!("bin");
//         total += 1.0/t * bin.len() as f64;
//     }

//     total
// }

pub fn calc_responsive_maintainer_summation(commits:Vec<RepoCommit>, t:f64) -> f64{
    let mut total:f64 = 0.0;

    total += 1.0/t * commits.len() as f64;

    total
}

pub fn calc_responsive_maintainer(weight_factor:f64, continuous_integration:bool, summation:f64, avg_time:f64) -> f64
{
    let mut integration: f64 = 0.0;
    if(continuous_integration){
        integration = 1.0;
    }

    let mut score:f64 = 0.0;
    score = weight_factor * integration + summation + (1.0/avg_time);

    let max_score = 100.0;
    if score > 100.0{
        score = 100.0;
    }
    else if score < 0.0{
        score = 0.0;
    }

    println!("Score: {}", score);
    return score;
}

pub fn calc_commit_bin_size(k: f64, repo:Repository)-> f64{
    let current_time = Utc::now();
    let mut duration = current_time - current_time;
    let mut t = 0.0;

    if let Some(repo) = repo.created_at {
        duration = current_time - repo;
    }

    //println!("Duration: {}", duration.num_days());

    //let t = 100.0 * f64::exp(-k * duration.num_days() as f64);

    // let max_t:f64 = 1000.0;
    // let normalized_t = t / max_t;
    // if duration.num_days() < 365
    // {
    //     t = 14.0;
    // }
    // else if duration.num_days() < 730
    // {
    //     t = 21.0;
    // }
    // else{
    //     t = 28.0;
    // }
    let t = duration.num_days() as f64;
    t
}

pub fn calc_duration_between_first_and_last_commit(commits:Vec<RepoCommit>) -> f64{
    println!("commits stat: {:#?}", commits[0].sha);
    
    0.0
}
