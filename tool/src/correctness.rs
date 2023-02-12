
pub async fn get_weighted_score(repo: &str, client: &Client) -> Result<f64, Error> {
    let first = are_all_issues_closed(repo, client).await?;
    let second = has_testing_suite(repo, client).await?;
    let third = check_number_of_releases(repo, client).await?;

    let total_score = (first as f64 / 3.0) + (second as f64 / 3.0) + (third as f64 / 3.0);

    Ok(total_score)
}