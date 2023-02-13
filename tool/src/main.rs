#![allow(unused)]

use clap::Parser;
use log::{info, warn, debug};
use std::io::{self, Write};
use anyhow::{Context, Result};

use dotenv::dotenv;
use std::env;

mod octo;
mod calc_responsive_maintainer;


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

    // let owner = "MinecraftForge";
    // let repo_name = "ForgeGradle";
    // let owner = "nullivex";
    // let repo_name = "nodist";
    // let owner = "cloudinary";
    // let repo_name = "cloudinary_npm";
    let owner = "lodash";
    let repo_name = "lodash";
    // let repo = octo::get_repo(token.clone(), owner.into(), repo_name.into()).await.unwrap();
    // let t = calc_responsive_maintainer::calc_commit_bin_size(0.1, repo.clone());
    // let binned_issues = octo::get_issues(token.clone(), owner.into(), repo_name.into(), t as i64).await.unwrap();
    // let average_duration: f64 = calc_responsive_maintainer::get_avg_issue_duration(binned_issues);


    let token: String = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required").into();
    //info!("Retrieved {}", repo.clone().name);

    // TODO optimize with BefReader    
    let content = std::fs::read_to_string(&args.path);
    let path = &args.path;
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;


    writeln!(handle_lock, "file content: {}", content);
   // writeln!(handle_lock, "{:#?}", repo.clone().license);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::calc_responsive_maintainer::calc_responsive_maintainer;

    use super::*;

    #[test]
    fn test_calc_responsive_maintainer() {
        let owner = "cloudinary";
        let repo_name = "cloudinary_npm";
        let expected_output = 0.0;
        let token: String = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required").into();

        let result = calc_responsive_maintainer::calc_responsive_maintainer(0.0, 0.0);
        assert_eq!(result, expected_output);
    }
}

