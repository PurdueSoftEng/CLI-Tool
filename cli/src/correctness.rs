extern crate octocrab;
use octocrab::{Octocrab, Error};
use crate::octo;

pub async fn get_weighted_score(token: String, owner: String, repo: String) -> Result<f64, Error> {
    let octo = Octocrab::builder().personal_token(token.clone()).build().unwrap();

    let first = octo::are_all_issues_closed(octo.clone(), owner.clone(), repo.clone()).await?;
    let second = octo::has_testing_suite(octo.clone(), owner.clone(), repo.clone()).await?;
    //let third = octo::check_number_of_releases(token.clone(), owner.clone(), repo.clone()).await?;
    let third = 0;

    let total_score = (first as f64 / 3.0) + (second as f64 / 3.0) + (third as f64 / 3.0);

    Ok(total_score)
}