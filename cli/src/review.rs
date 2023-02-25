use crate::reqw;

pub async fn calc_review(client: &reqwest::Client, owner: &str, repo: &str) -> f32 {
    let json = reqw::graph_json(client, 
        format!("{{\"query\" : \"query {{ search(query: \\\"repo:{}/{} is:pr is:merged\\\" first:100 type:ISSUE) {{ nodes {{ ... on PullRequest {{ reviewDecision additions deletions }} }} }} }}\" }}", owner, repo)
        ).await.unwrap();

    let mut lines = 0;
    let mut approved = 0;
    for pr in json["data"]["search"]["nodes"].as_array().unwrap() {
        let change = pr["additions"].as_i64().unwrap() + pr["deletions"].as_i64().unwrap();
        lines += change;
        if pr["reviewDecision"] == "APPROVED" {
            approved += change;
        }
    }

    if lines == 0 {
        0.0
    } else {
        approved as f32 / lines as f32
    }
}
