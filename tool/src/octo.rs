extern crate octocrab;
use octocrab::{Octocrab, Page, Result, models, params};

#[allow(non_snake_case)]
pub fn initOcto(token: String) -> Result<Octocrab, octocrab::Error>
{
    (Octocrab::builder().personal_token(token).build())
}

#[allow(non_snake_case)]
pub async fn getRepo(token: String, owner: String, repo: String) -> octocrab::models::Repository
{
    let octo = Octocrab::builder().personal_token(token).build().unwrap();
    match octo.repos(owner, repo).get().await
    {
        Ok(repo) => repo,
        Err(_) => panic!("Error fetching repo"),
    }
}

#[allow(non_snake_case)]
pub async fn getIssue(token: String, owner: String, repo: String) -> Result<Page<octocrab::models::issues::Issue>, octocrab::Error> {
    let route = format!("repos/{owner}/{repo}/issues/");
    let octo = Octocrab::builder().personal_token(token).build().unwrap();

    println!("Owner: {}", owner);
    println!("Repo: {}", repo);
    
    let mut page = octo.issues(owner, repo).list().state(params::State::All).send().await?;
    let mut i = 0;
    loop {
        for issue in &page.items {
            println!("{}: {} -- {}", i, issue.title, issue.created_at);
            i += 1;
        }
        page = match octo
            .get_page::<models::issues::Issue>(&page.next)
            .await
        {
            Ok(Some(next_page)) => next_page,
            Ok(None) => break,
            Err(_) => break,
        }
    }

    Ok(page)
}
// pub async fn getIssue(token: String, owner: String, repo: String) -> Page<octocrab::models::issues::Issue>
// {
//     let route = format!("repos/{owner}/{repo}/issues/");
//     let octo = Octocrab::builder().personal_token(token).build().unwrap();

//     println!("Owner: {}", owner);
//     println!("Repo: {}", repo);
//     //match octo.issues(owner, repo).list().send().await

    
//     let mut page = octo.issues(owner, repo).list().send().await;

//     loop {
//         for issue in &page {
//             println!("{}", issue.title);
//         }
//         page = match octo
//             .get_page::<models::issues::Issue>(&page.next)
//             .await
//         {
//             Ok(Some(next_page)) => next_page,
//             Ok(None) => break,
//             Err(_) => break,
//         }
//     }
// }

    // match octo.issues(owner, repo).list().send().await
    // {
    //     Ok(page) => page,
    //     Err(_) => panic!("Error fetching issue"),
    // }


// pub async fn getAllIssues(token: String, owner: String, repo: String) -> Vec<octocrab::models::issues::Issue> {
//     //let route = format!("repos/{owner}/{repo}/issues/");
//     let route = format!("repos/{}/{}/issues/", owner, repo);
//     let octo = Octocrab::builder().personal_token(token).build().unwrap();
//     let mut page_number = 1;
//     let mut all_issues = vec![];

//     loop {
//         let url = format!("{}?page={}&per_page=100", route, page_number);
//         //let mut page = match octo.get::<Page<octocrab::models::issues::Issue>, &str, &str>(&url, Some(&route.as_str())).await {
//         let mut page = match octo.get::<Page<octocrab::models::issues::Issue>, &str, &str>(&route, Some(&url.as_str())).await {
//             Ok(page) => page,
//             Err(error) => panic!("Error fetching issue: {}", error),
//         };

//         if page.clone().into_iter().len() == 0 {
//             break;
//         }

//         all_issues.extend(page.clone().into_iter().map(|issue| issue.clone()));
//         page_number += 1;
//     }

//     all_issues
// }

// pub async fn getAllIssues(token: String, owner: String, repo: String) -> Vec<octocrab::models::issues::Issue> {
//     let route = format!("repos/{owner}/{repo}/issues/", owner=owner, repo=repo);
//     let octo = Octocrab::builder().personal_token(token).build().unwrap();
//     let mut page_number = 1;
//     let mut all_issues = vec![];

//     loop {
//         let url = format!("{}?page={}&per_page=100", route, page_number);
//         let response = octo.get(&url, Some(&route.as_str())).await;
//         match response {
//             Ok(response) => {
//                 println!("Response: {:?}", response.text().await);
//                 let mut page = response.json::<Page<octocrab::models::issues::Issue>>().await.unwrap();
//                 if page.clone().into_iter().len() == 0 {
//                     break;
//                 }

//                 all_issues.extend(page.clone().into_iter().map(|issue| issue.clone()));
//                 page_number += 1;
//             },
//             Err(error) => panic!("Error fetching issue: {}", error),
//         }
//     }

//     all_issues
// }

// pub async fn getIssue(token: String, owner: String, repo: String) -> Vec<octocrab::models::issues::Issue>
// {
//     let route = format!("repos/{owner}/{repo}/issues/");
//     let octo = Octocrab::builder().personal_token(token).build().unwrap();

//     println!("Owner: {}", owner);
//     println!("Repo: {}", repo);
//     let mut page_number = 1;
//     let mut all_issues = vec![];

//     loop {
//         let url = format!("{}?page={}&per_page=100", route, page_number);
//         let page = match octo.get::<Page<octocrab::models::issues::Issue>, &str, &str>(&url, Some(&route.as_str())).await {            Ok(page) => page,
//             Err(_) => panic!("Error fetching issue"),
//         };

//         if page.clone().into_iter().len() == 0 {
//             break;
//         }

//         all_issues.extend(page.clone().into_iter().map(|issue| issue.clone()));
//         page_number += 1;
//     }

//     all_issues
// }



