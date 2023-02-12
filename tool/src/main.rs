#![allow(unused)]

use clap::Parser;
use log::{info, warn, debug, LevelFilter};
use std::io::{self, Write};
use anyhow::{Context, Result};

use dotenv::dotenv;
use std::env;

mod octo;
mod calc_responsive_maintainer;

#[derive(Parser)]
struct Cli {
    path: String,
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
    let token: String = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required").into();

    // let owner = "MinecraftForge";
    // let repo_name = "ForgeGradle";
    let owner = "PurdueSoftEng";
    let repo_name = "CLI-tool";
    let repo = octo::get_repo(token.clone(), owner.into(), repo_name.into()).await.unwrap();

    let t = calc_responsive_maintainer::calc_commit_bin_size(0.1, repo.clone());
    let binned_issues = octo::get_issues(token.clone(), owner.into(), repo_name.into(), t as i64).await.unwrap();
    let average_duration: f64 = calc_responsive_maintainer::get_avg_issue_duration(binned_issues);
    let commit_pages = octo::get_all_commits(token.clone(), owner.into(), repo_name.into()).await.unwrap();
    //let temp = calc_responsive_maintainer::calc_duration_between_first_and_last_commit(commit_pages.clone());
    //let t = octo::get_duration_between_first_and_last_commit(token.clone(), owner.into(), repo_name.into()).await.unwrap();
    //let t = 0.0;
    let responsive_maintainer_summation: f64 = calc_responsive_maintainer::calc_responsive_maintainer_summation(commit_pages, t);
    let uses_workflows = octo::uses_workflows(token.clone(), owner.into(), repo_name.into()).await.unwrap();

    calc_responsive_maintainer::calc_responsive_maintainer(1.0, uses_workflows, responsive_maintainer_summation, average_duration);

    let repo2 = octo::get_repo(token.clone(), "rust-lang".into(), "rust".into()).await.unwrap();
    info!("Retrieved {}", repo2.name);

    writeln!(handle_lock, "{:#?}", repo2.license);

    let mut resp = octo::get_license(token.clone(), "rust-lang".into(), "rust".into()).await;
    let data_layer = resp.get_mut("data").expect("Data key not found");
    let repository_layer = data_layer.get_mut("repository").expect("Repository key not found");
    let license_layer = repository_layer.get_mut("licenseInfo").expect("License key not found");
    writeln!(handle_lock, "{:#?}", license_layer.get("name").as_str().unwrap());

    Ok(())
}


