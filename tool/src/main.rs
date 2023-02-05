#![allow(unused)]

use clap::Parser;
use log::{info, warn};
use octocrab::Page;
use std::io::{self, Write};
use anyhow::{Context, Result};

use crate::octo::initOcto;

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
    let args = Cli::parse();
    let stdout = io::stdout();
    let mut handle_lock = stdout.lock();
    let token: String = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required").into();
    let repo = octo::getRepo(token.clone(), "MinecraftForge".into(), "ForgeGradle".into()).await;

    let binnedIssues = octo::getIssues(token.clone(), "PurdueSoftEng".into(), "CLI-Tool".into(), 2).await;

    let averageDuration: f64 = octo::getAvgIssueDuration(binnedIssues.unwrap());
    //let page = octo::getAllIssues(token.clone(), "microsoft".into(), "vscode".into()).await;

    // TODO optimize with BefReader    
    let content = std::fs::read_to_string(&args.path);
    let path = &args.path;
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;

    calcResponsiveMaintainer(1.0, 1.0, 1.0, averageDuration, 2.0);

    writeln!(handle_lock, "file content: {}", content);
    
    //writeln!(handle_lock, "{:#?}", repo.unwrap().license);
    
    Ok(())

}

#[allow(non_snake_case)]
fn calcResponsiveMaintainer(weightFactor:f64, continuousIntegration:f64, summation:f64, avgTime:f64, t:f64) -> f64
{
    let mut score:f64 = 0.0;
    score = weightFactor * continuousIntegration + summation + (1.0/avgTime);

    println!("Score: {}", score);
    return score;
}

