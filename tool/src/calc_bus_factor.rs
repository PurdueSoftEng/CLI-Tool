use crate::octo;

// This is a helper function that sorts our contributors vector so that the contributors with the
// highest percentage of contributions come first. This essentially ranks the percentages from 
// highest to lowest.
pub fn sort_by_percentage(contributors: &mut Vec<(octocrab::models::repos::Contributor, u32, f32)>) {
    contributors.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
}

// This function finds our core contributors. Our group decided that we would add up the highest
// contrubtors until we reached 80%. We take the index + 1, or the amount of contributors
// it took to get to this point. This is our number of core contributors.
pub fn find_core_contributors(contributors: &[(octocrab::models::repos::Contributor, u32, f32)]) -> usize {
    let mut cumulative_percentage = 0.0;

    for (index, &(_, _, percentage)) in contributors.iter().enumerate() {
        cumulative_percentage += percentage;
        if cumulative_percentage >= 0.8 {
            return index + 1; //number of core contributers who add up to 80%
        }
    }

    contributors.len()
}

// Here we are creating a function to get the factor ratio. This is the number of core contributors
// divided by the total number of contributors. 
pub fn get_factor_ratio(contributors: &[(octocrab::models::repos::Contributor, u32, f32)], top_index: usize) -> f32 {
    let total_contributors = contributors.len();
    top_index as f32 / total_contributors as f32
}

// Our group decided that we valued expertise more than many people working the same amount on a 
// repository. For this metric we decided we would look at the factor ratio we calculated. For
// example if we had a repository with 10 contributors but only 1 person did 80% of the work this
// is better than 5 people doing 80% of the work. To better score our metric we do 1 - the factior
// ratio to get a decimal to score it.
pub async fn calculate_bus_factor(token: String, owner: String, repo: String) -> f32 {

    let mut contributors = octo::get_contributors_with_percentages(token, owner, repo).await.unwrap();
    
    let mut converted_contributors: Vec<(octocrab::models::repos::Contributor, u32, f32)> = contributors
    .into_iter()
    .map(|(contributor, commits, percent)| (contributor, commits as u32, percent))
    .collect();
    
    sort_by_percentage(&mut converted_contributors);
    let core_contributors = find_core_contributors(&converted_contributors);
    let factor_ratio = get_factor_ratio(&converted_contributors, core_contributors);
    1.0 - factor_ratio
}


