extern crate octocrab;
use octocrab::{Octocrab, Error};
use crate::octo;


// Our group determined that the presense of a README was the most important part of
// the ramp up score. A README allows others to get versed in a project and learn what
// its about. If there is a README the repository will recieve a score of 1. If not
// it will get a score of 0.
pub async fn get_weighted_score(octo: Octocrab, owner: String, repo: String) -> Result<f64, Error> {
    let first = octo::check_readme(octo.clone(), owner.clone(), repo.clone()).await?;

    if first >= 2 {
        Ok(1.0)
    } else if first == 1 {
        Ok(0.5)
    } else {
        Ok(0.0)
    }
}
