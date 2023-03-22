#![allow(unused)]

use clap::Parser;
use log::{info, warn, debug, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use std::io::{self, Write};
use anyhow::{Context, Result};
extern crate octocrab;

use octocrab::{Octocrab, Page, models::{self, repos::RepoCommit}, params};

use serde_json::{json, Value, Map};

use dotenv::dotenv;
use std::env;
use std::path::Path;

use regex::Regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::cmp::Ordering;

mod octo;
mod calc_responsive_maintainer;
mod correctness;
mod calc_bus_factor;
mod ramp_up;

extern crate serde;
extern crate serde_json;

use std::io::prelude::*;
mod calc_license;

#[derive(Parser)]
struct Cli {
    path: String,
}

#[derive(Debug)]
struct CustomError(String);

/*#[derive(Serialize, Deserialize, Debug)]
struct Output {

}*/

// #[derive(Debug)]
// struct GithubRepo {
//     url: String,
//     scores: Vec<f32>,
// }
#[derive(Debug, Clone)]
struct GithubRepo {
    url: String,
    scores: Vec<f32>,
}

impl GithubRepo {
    fn new(url: String, scores: Vec<f32>) -> Self {
        GithubRepo {
            url,
            scores,
        }
    }

    fn overall(&self) -> f32 {
        self.scores[0]
    }

    fn overall_set(&mut self, overall_score: f32){
        self.scores[0] = overall_score;
    }

    fn bus(&self) -> f32 {
        self.scores[1]
    }
    
    fn bus_set(&mut self, bus_score: f32) {
        self.scores[1] = bus_score;
    }

    fn correct(&self) -> f32 {
        self.scores[2]
    }

    fn correct_set(&mut self, correct_score: f32) {
         self.scores[2] = correct_score;
    }

    fn license(&self) -> f32 {
        self.scores[3]
    }

    fn license_set(&mut self, license_score: f32) {
        self.scores[3] = license_score;
    }

    fn responsive(&self) ->f32 {
        self.scores[4]
    }

    fn responsive_set(&mut self, responsive_score: f32) {
        self.scores[4] = responsive_score;
    }

    fn rampup(&self) -> f32{
        self.scores[5]
    }

    fn rampup_set(&mut self, rampup_score: f32) {
        self.scores[5] = rampup_score;
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
        let scores = vec![-1.0, -1.0, -1.0, -1.0, -1.0, -1.0];
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
    //env_logger::init();
    dotenv().ok();
    let args = Cli::parse();
    let stdout = io::stdout();
    let mut handle_lock = stdout.lock();
    let token: String = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required").into();
    let log_file = env::var("LOG_FILE").expect("LOG_FILE environment variable not set");
    let log_level = env::var("LOG_LEVEL").expect("LOG_LEVEL environment variable not set");
    let log_path = Path::new(&log_file);

    let mut logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(log_path)?;

    let log_level = match log_level.as_str() {
        "1" => LevelFilter::Error,
        "2" => LevelFilter::Warn,
        "3" => LevelFilter::Info,
        _ => panic!("Invalid log level specified: {}", log_level),
    };

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(log_level))?;

    log4rs::init_config(config)?;

    info!("Logger initialized");

    let mut repos_list = read_github_repos_from_file(&args.path);
    //println!("{}", repos_list);
    let repo_info = extract_owner_and_repo(repos_list.first().unwrap().url.as_str());
    let owner = repo_info.clone().unwrap().0;
    let repo_name = repo_info.clone().unwrap().1;

    let repo = octo::get_repo(token.clone(), owner.clone(), repo_name.clone()).await;

    for repository in &mut repos_list{
        calc_metrics(repository, token.clone(), owner.clone(), repo_name.clone()).await;
        //sort_repositories(repos_list.as_mut());
        create_ndjson(repository.url.as_str(), repository.overall(), repository.rampup(), repository.correct(), repository.bus(), repository.responsive(), repository.license());
    }

    Ok(())
    
}

async fn calc_metrics(repository: &mut GithubRepo, token: String, owner: String, repo: String) {
    let mut issue_response_times = octo::get_issue_response_times(token.clone(), owner.clone(), repo.clone()).await.unwrap();
    let mut responsive_score = calc_responsive_maintainer::calc_responsive_maintainer(issue_response_times[0], issue_response_times[1]) as f32;
    repository.responsive_set(responsive_score);

    let mut resp = octo::get_license(token.clone(), owner.clone().as_str(), repo.clone().as_str()).await;
    let data_layer = resp.get_mut("data").expect("Data key not found");
    let repository_layer = data_layer.get_mut("repository").expect("Repository key not found");
    let license_layer = repository_layer.get_mut("licenseInfo").expect("License key not found");
    let mut license_score = 0;
    if license_layer.get("key").is_some()
    {
        license_score = calc_license::calc_licenses(license_layer.get("key").unwrap().to_string()).await;
        println!("license score: {}", license_score);
    }
    repository.license_set(license_score.into());

    let octo = Octocrab::builder().personal_token(token.clone()).build().unwrap();

    let mut ramp_up_score = ramp_up::get_weighted_score(octo.clone(), owner.clone(), repo.clone()).await.unwrap();
    //println!("ramp up score: {}", ramp_up_score);
    repository.rampup_set(ramp_up_score as f32);

    let mut correctness_score = correctness::get_weighted_score(token.clone(), owner.clone(), repo.clone()).await.unwrap();
    //println!("correctness: {}", correctness_score);
    //let correctness_score = 0;
    repository.correct_set(correctness_score as f32);

    let mut bus_factor_score = 0;
    //let bus_factor_score = calc_bus_factor::calculate_bus_factor(token.clone(), owner.clone(), repo.clone()).await;
    repository.bus_set(bus_factor_score as f32);

    let mut net_score_score = 0.0;
    repository.overall_set(net_score_score);
}

fn create_ndjson(url: &str, net_score: f32, ramp_up_score: f32, correctness_score: f32, bus_factor_score: f32, responsive_maintainer_score: f32, license_score: f32) {
    let json = json!({
        "URL": url,
        "NET_SCORE": format!("{:.1}", net_score),
        "RAMP_UP_SCORE": format!("{:.1}", ramp_up_score),
        "CORRECTNESS_SCORE": format!("{:.1}", correctness_score),
        "BUS_FACTOR_SCORE": format!("{:.1}", bus_factor_score),
        "RESPONSIVE_MAINTAINER_SCORE": format!("{:.1}", responsive_maintainer_score),
        "LICENSE_SCORE": format!("{:.1}", license_score),
    });
    let ndjson = json.to_string();
    println!("{}", ndjson);
}

#[cfg(test)]
mod tests {
    use crate::calc_responsive_maintainer::calc_responsive_maintainer;
    use std::fs::File;
    use std::io::prelude::*;
    use tempfile::tempdir;
    use super::*;

    #[test]
    fn test_calc_responsive_maintainer() {
        let owner = "cloudinary";
        let repo_name = "cloudinary_npm";
        let expected_output = 0.0;
        //let token: String = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required").into();

        let result = calc_responsive_maintainer::calc_responsive_maintainer(0.0, 0.0);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_read_github_repos_from_file() {
        let temp_dir = tempdir().unwrap();
        let test_file_path = temp_dir.path().join("test_file.txt");
        let mut file = File::create(&test_file_path).unwrap();
        file.write_all(b"https://github.com/lodash/lodash\nhttps://github.com/nullivex/nodist\nhttps://www.npmjs.com/package/browserify").unwrap();

        let repos = read_github_repos_from_file(test_file_path.to_str().unwrap());
        assert_eq!(repos.len(), 3);
        assert_eq!(repos.get(0).unwrap().url, String::from("https://github.com/lodash/lodash"));
        assert_eq!(repos.get(1).unwrap().url, String::from("https://github.com/nullivex/nodist"));
        assert_eq!(repos.get(2).unwrap().url, String::from("https://www.npmjs.com/package/browserify"));
    }
}

// fn sort_repositories(repositories: &mut Vec<GithubRepo>) {
//     repositories.sort_by(|a, b| {
//         let overall_cmp = b.overall().cmp(&a.overall());
//         if overall_cmp == Ordering::Equal {
//             let bus_cmp = b.bus().cmp(&a.bus());
//             if bus_cmp == Ordering::Equal {
//                 a.license().cmp(&b.license())
//             } else {
//                 bus_cmp
//             }
//         } else {
//             overall_cmp
//         }
//     });
// }
