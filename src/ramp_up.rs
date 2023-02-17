extern crate octocrab;
use octocrab::{Octocrab, Error};
use crate::octo;


// Our group determined that the presense of a README was the most important part of
// the ramp up score. A README allows others to get versed in a project and learn what
// its about. If there is a README the repository will recieve a score of 1. If not
// it will get a score of 0.
pub async fn get_weighted_score(octo: Octocrab, owner: String, repo: String) -> Result<f64, Error> {
    let first = octo::has_readme(octo.clone(), owner.clone(), repo.clone()).await?;
    let second = octo::check_multiple_readmes(octo.clone(), owner.clone(), repo.clone()).await?;

    let total_score = (first as f64 / 2.0) + (second as f64 / 2.0);

    Ok(total_score)
}