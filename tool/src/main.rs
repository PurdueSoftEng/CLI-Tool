#![allow(unused)]

use clap::Parser;
use log::{info, warn};
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
    let repo = octo::getRepo(token.clone(), "rust-lang".into(), "rust".into()).await;
    //let page = octo::getIssue(token.clone(), "rust-lang".into(), "rust".into()).await;
    //let page = octo::getIssue(token.clone(), "PurdueSoftEng".into(), "CLI-Tool".into()).await;
    let page = octo::getIssue(token.clone(), "MinecraftForge".into(), "ForgeGradle".into()).await;
    //let page = octo::getAllIssues(token.clone(), "microsoft".into(), "vscode".into()).await;

    
    
    // TODO optimize with BefReader    
    let content = std::fs::read_to_string(&args.path).unwrap();
    let path = &args.path;
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;

    let mut elements = vec![];
    elements.push("1/10/2020");
    elements.push("1/11/2020");
    calcResponsiveMaintainer(1.0, 1.0, 1.0, 10.0);

    writeln!(handle_lock, "file content: {}", content);
    writeln!(handle_lock, "{:#?}", repo.license);

    Ok(())

}

#[allow(non_snake_case)]
fn calcResponsiveMaintainer(weightFactor:f32, continuousIntegration:f32, summation:f32, avgTime:f32) -> f32
{
    let mut score:f32 = 0.0;

    let mut i = 0;
    // for issue in issues
    // {
    //     println!("{}: {}", i, issue.created_at);
    //     i += 1;
    // }
    // for i in (0..t){
    //     score+=1.0;
    // }
    score = weightFactor * continuousIntegration + summation + (1.0/avgTime);
    println!("Score: {}", score);
    return score;
}

