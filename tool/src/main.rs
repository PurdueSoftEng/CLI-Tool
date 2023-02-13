#![allow(unused)]

use clap::Parser;
use log::{info, warn, debug, LevelFilter};
use std::io::{self, Write};
use std::ops::IndexMut;
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

fn sort_repositories(repositories: &mut Vec<GithubRepo>) {
    repositories.sort_by(|a, b| {
        let overall_cmp = b.overall().cmp(&a.overall());
        if overall_cmp == Ordering::Equal {
            let bus_cmp = b.bus().cmp(&a.bus());
            if bus_cmp == Ordering::Equal {
                a.license().cmp(&b.license())
            } else {
                bus_cmp
            }
        } else {
            overall_cmp
        }
    });
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
    let mut repos_list = read_github_repos_from_file(&args.path);
    for repository in repos_list.as_mut_slice()
    {
        let repo_info = extract_owner_and_repo(&repository.url);
        let owner = repo_info.clone().unwrap().0;
        let repo_name = repo_info.clone().unwrap().1;
        let repo = octo::get_repo(token.clone(), owner.clone(), repo_name.clone()).await;

        let mut resp = octo::get_license(token.clone(), owner.clone().as_str(), repo_name.clone().as_str()).await;
        let data_layer = resp.get_mut("data").expect("Data key not found");
        let repository_layer = data_layer.get_mut("repository").expect("Repository key not found");
        let license_layer = repository_layer.get_mut("licenseInfo").expect("License key not found");
        if license_layer.is_null()
        {
            //let license_score = calc_license::calc_licenses(license_layer.get("key").unwrap().to_string()).await;
            let license_score = 0;
        }
        else
        {
            let license_score = calc_license::calc_licenses(license_layer.get("key").unwrap().to_string()).await;
            //let license_score = 0;
        }
    }

    sort_repositories(repos_list.as_mut());
    //info!("Retrieved {}", repo.name);

    Ok(())
}


