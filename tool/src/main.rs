#![allow(unused)]

use clap::Parser;
use log::{info, warn, debug};
use std::io::{self, Write};
use anyhow::{Context, Result};

use dotenv::dotenv;
use std::env;

mod octo;


#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenv().ok();
    let args = Cli::parse();
    let stdout = io::stdout();
    let mut handle_lock = stdout.lock();
    let token: String = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required").into();
    let repo = octo::get_repo(token.clone(), "MinecraftForge".into(), "ForgeGradle".into()).await.unwrap();

    let binned_issues = octo::get_issues(token.clone(), "PurdueSoftEng".into(), "CLI-Tool".into(), 2).await.unwrap();

    let average_duration: f64 = octo::get_avg_issue_duration(binned_issues);
    //let page = octo::getAllIssues(token.clone(), "microsoft".into(), "vscode".into()).await;

    let token: String = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required").into();
    
    let repo = octo::get_repo(token.clone(), "rust-lang".into(), "rust".into()).await.unwrap();
    info!("Retrieved {}", repo.name);
    let page = octo::get_issue(token.clone(), "rust-lang".into(), "rust".into()).await.unwrap();

    // TODO optimize with BefReader    
    let content = std::fs::read_to_string(&args.path);
    let path = &args.path;
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;

    calc_responsive_maintainer(1.0, 1.0, 1.0, average_duration, 2.0);

    writeln!(handle_lock, "file content: {}", content);

    writeln!(handle_lock, "{:#?}", repo.license);
    for issue in page
    {
        writeln!(handle_lock, "{:#?}", issue.closed_at);
    }
    let resp = octo::get_num_commits(token.clone(), "rust-lang".into(), "rust".into()).await;
    writeln!(handle_lock, "Query: {:#?}", resp);
    Ok(())

}

#[allow(non_snake_case)]
fn calc_responsive_maintainer(weight_factor:f64, continuous_integration:f64, summation:f64, avg_time:f64, t:f64) -> f64
{
    let mut score:f64 = 0.0;
    score = weight_factor * continuous_integration + summation + (1.0/avg_time);

    println!("Score: {}", score);
    return score;
}

