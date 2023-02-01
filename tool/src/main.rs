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

    writeln!(handle_lock, "file content: {}", content);
    writeln!(handle_lock, "{:#?}", repo.license);
    for issue in page
    {
        writeln!(handle_lock, "{}", issue.title);
    }
    Ok(())
}


