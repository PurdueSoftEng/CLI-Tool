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
    let page = octo::getIssue(token.clone(), "rust-lang".into(), "rust".into()).await;
    
    // TODO optimize with BefReader    
    let content = std::fs::read_to_string(&args.path).unwrap();
    let path = &args.path;
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;

    calcResponsiveMaintainer(1, 1, 1.0, 10.0);

    writeln!(handle_lock, "file content: {}", content);
    writeln!(handle_lock, "{:#?}", repo.license);
    for issue in page
    {
        writeln!(handle_lock, "{}", issue.title);
    }
    Ok(())

}

#[allow(non_snake_case)]
// For ResponsiveMaintainer and measuring the continuous integration we will use the equation:
//  WF *CI+1ti=0tc(t) +1/(avg time)
// The timescale (t) can be dynamic depending on further analysis. The weight factor (WF) will be 
// later tuned depending on the importance we choose to place on projects with continuous integration. 
// Continuous integration (CI) is a binary value based entirely on if the project has workflows or not.  
// We will be using an average time metric to see on average how long issues are open, in which we 
// add the inverse.
fn calcResponsiveMaintainer(weightFactor:i32, continuousIntegration:i32, t:f32, avgTime:f32) -> f32
{
    let score:f32 = 0.0;
    

    return score;
}

