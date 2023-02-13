#![allow(unused)]

use clap::Parser;
use log::{info, warn, debug, LevelFilter};
use std::io::{self, Write};
use anyhow::{Context, Result};

use dotenv::dotenv;
use std::env;

use regex::Regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::cmp::Ordering;

mod octo;
mod calc_responsive_maintainer;
mod calc_license;

#[derive(Parser)]
struct Cli {
    path: String,
}

#[derive(Debug)]
struct CustomError(String);

#[derive(Debug)]
struct GithubRepo {
    url: String,
    scores: Vec<i16>,
}

impl GithubRepo {
    fn new(url: String, scores: Vec<i16>) -> Self {
        GithubRepo {
            url,
            scores,
        }
    }

    fn overall(&self) -> i16 {
        self.scores[0]
    }

    fn bus(&self) -> i16 {
        self.scores[1]
    }

    fn correct(&self) -> i16 {
        self.scores[2]
    }

    fn license(&self) -> i16 {
        self.scores[3]
    }

    fn responsive(&self) -> i16 {
        self.scores[4]
    }

    fn rampup(&self) -> i16 {
        self.scores[5]
    }
}

fn read_github_repos_from_file(filename: &str) -> Vec<GithubRepo> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            println!("Error opening file: {}", err);
            return vec![];
        }
    };

    let reader = BufReader::new(file);

    let mut repos = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let scores = vec![-1, -1, -1, -1, -1, -1];
        let repo = GithubRepo::new(line, scores);
        repos.push(repo);
    }

    repos
}

fn extract_owner_and_repo(url: &str) -> Option<(String, String)> {
    let re = Regex::new(r"https://github.com/([^/]+)/([^/]+)/?").unwrap();
    let captures = re.captures(url)?;

    Some((captures[1].to_string(), captures[2].to_string()))
}

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
    let repos_list = read_github_repos_from_file(&args.path);
    let repo_info = extract_owner_and_repo(repos_list.first().unwrap().url.as_str());
    let owner = repo_info.clone().unwrap().0;
    let repo_name = repo_info.clone().unwrap().1;
    let repo = octo::get_repo(token.clone(), owner.clone(), repo_name.clone()).await;
    //info!("Retrieved {}", repo.name);

    let mut resp = octo::get_license(token.clone(), owner.clone().as_str(), repo_name.clone().as_str()).await;
    let data_layer = resp.get_mut("data").expect("Data key not found");
    let repository_layer = data_layer.get_mut("repository").expect("Repository key not found");
    let license_layer = repository_layer.get_mut("licenseInfo").expect("License key not found");
    if license_layer.get("key").is_some()
    {
        let license_score = calc_license::calc_licenses(license_layer.get("key").unwrap().to_string()).await;
    }
    else
    {
        let license_score = 0;
    }

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

