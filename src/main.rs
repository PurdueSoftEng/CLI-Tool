#![allow(unused)]

use clap::Parser;
use log::{info, warn, debug, LevelFilter};
use std::io::{self, Write};
use anyhow::{Context, Result};
extern crate octocrab;

use octocrab::{Octocrab, Page, models::{self, repos::RepoCommit}, params};

use serde_json::{json, Value, Map};

use dotenv::dotenv;
use std::env;

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

#[derive(Debug)]
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
    env_logger::init();
    dotenv().ok();
    let args = Cli::parse();
    let stdout = io::stdout();
    let mut handle_lock = stdout.lock();
    let token: String = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required").into();
    let mut repos_list = read_github_repos_from_file(&args.path);
    let repo_info = extract_owner_and_repo(repos_list.first().unwrap().url.as_str());
    let owner = repo_info.clone().unwrap().0;
    let repo_name = repo_info.clone().unwrap().1;

    let repo = octo::get_repo(token.clone(), owner.clone(), repo_name.clone()).await;

    for mut repository in repos_list{
        let scores_list = calc_metrics(token.clone(), owner.clone(), repo_name.clone()).await;
        for score in scores_list{
            repository.responsive_set(score);
            repository.license_set(score);
            repository.rampup_set(score);
            repository.correct_set(score);
            repository.bus_set(score);
            repository.overall_set(score);
        }
        
        create_ndjson(repository.url.as_str(), repository.overall(), repository.rampup(), repository.correct(), repository.bus(), repository.responsive(), repository.license());
    }

    Ok(())
    
}

async fn calc_metrics(token: String, owner: String, repo: String) -> Vec<f32> {
    let mut scores_vec = Vec::new();
    let mut issue_response_times = octo::get_issue_response_times(token.clone(), owner.clone(), repo.clone()).await.unwrap();
    let mut responsive_score = calc_responsive_maintainer::calc_responsive_maintainer(issue_response_times[0], issue_response_times[1]) as f32;
    scores_vec.push(responsive_score);

    let mut resp = octo::get_license(token.clone(), owner.clone().as_str(), repo.clone().as_str()).await;
    let data_layer = resp.get_mut("data").expect("Data key not found");
    let repository_layer = data_layer.get_mut("repository").expect("Repository key not found");
    let license_layer = repository_layer.get_mut("licenseInfo").expect("License key not found");
    let mut license_score = 0;
    if license_layer.get("key").is_some()
    {
        license_score = calc_license::calc_licenses(license_layer.get("key").unwrap().to_string()).await;
    }
    scores_vec.push(license_score as f32);

    let octo = Octocrab::builder().personal_token(token).build().unwrap();

    let ramp_up_score = ramp_up::get_weighted_score(octo.clone(), owner.clone(), repo.clone()).await.unwrap();
    scores_vec.push(ramp_up_score as f32);

    let correctness_score = correctness::get_weighted_score(octo.clone(), owner.clone(), repo.clone()).await.unwrap();
    scores_vec.push(correctness_score as f32);

    let bus_factor_score = calc_bus_factor::calculate_bus_factor(token.clone(), owner.clone(), repo.clone()).await;
    scores_vec.push(bus_factor_score as f32);

    let net_score_score = 0.0;
    scores_vec.push(net_score_score as f32);

    scores_vec   
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

use serde::ser::{Serialize, SerializeMap, Serializer};

struct NdjsonSerializer<'a> {
    url: &'a str,
    net_score: f32,
    ramp_up_score: f32,
    correctness_score: f32,
    bus_factor_score: f32,
    responsive_maintainer_score: f32,
    license_score: f32,
}

impl<'a> Serialize for NdjsonSerializer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(7))?;
        map.serialize_entry("URL", self.url)?;
        map.serialize_entry("NET_SCORE", &self.net_score)?;
        map.serialize_entry("RAMP_UP_SCORE", &self.ramp_up_score)?;
        map.serialize_entry("CORRECTNESS_SCORE", &self.correctness_score)?;
        map.serialize_entry("BUS_FACTOR_SCORE", &self.bus_factor_score)?;
        map.serialize_entry("RESPONSIVE_MAINTAINER_SCORE", &self.responsive_maintainer_score)?;
        map.serialize_entry("LICENSE_SCORE", &self.license_score)?;
        map.end()
    }
}

fn create_ndjson(url: &str, net_score: f32, ramp_up_score: f32, correctness_score: f32, bus_factor_score: f32, responsive_maintainer_score: f32, license_score: f32) {
    let serializer = NdjsonSerializer {
        url,
        net_score,
        ramp_up_score,
        correctness_score,
        bus_factor_score,
        responsive_maintainer_score,
        license_score,
    };
    let ndjson = serde_json::to_string(&serializer).unwrap();
    println!("{}", ndjson);
}
