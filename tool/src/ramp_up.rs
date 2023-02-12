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