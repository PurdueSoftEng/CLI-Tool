use octocrab::{Client, Error};

// Our group determined that the presense of a README was the most important part of
// the ramp up score. A README allows others to get versed in a project and learn what
// its about. If there is a README the repository will recieve a score of 1. If not
// it will get a score of 0.
async fn has_readme(repo: &str, client: &Client) -> Result<i32, Error> {
    let file_list_url = format!("/repos/{}/git/trees/master?recursive=1", repo);
    let file_list_response = client
        .get(file_list_url)
        .send()
        .await?;

    let file_list_content = file_list_response.json::<serde_json::Value>().await?;
    let files = file_list_content["tree"].as_array().unwrap();

    for file in files {
        let path = file["path"].as_str().unwrap();
        if path == "README.md" {
            return Ok(1);
        }
    }

    Ok(0)
}

async fn check_multiple_readmes(repo: &str, client: &Client) -> Result<i32, Error> {
    let contents_url = format!("/repos/{}/contents/", repo);
    let contents_response = client.get(contents_url).send().await?;

    let contents = contents_response.json::<Vec<serde_json::Value>>().await?;

    let mut readme_count = 0;

    for content in contents {
        let name = content["name"].as_str().unwrap();
        if name.to_lowercase().starts_with("readme") {
            readme_count += 1;
        }
    }

    if readme_count > 1 {
        Ok(1)
    } else {
        Ok(0)
    }
}

async fn get_weighted_score(repo: &str, client: &Client) -> Result<f64, Error> {
    let first = has_readme(repo, client).await?;
    let second = check_multiple_readmes(repo, client).await?;

    let total_score = (first as f64 / 2.0) + (second as f64 / 2.0);

    Ok(total_score)
}