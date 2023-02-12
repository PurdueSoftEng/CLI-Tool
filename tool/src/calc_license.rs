
// Rust function to get license metric result
// If a the license key matches a keyword from 
// the compatible GitHub licenses, print 1, else print 0 
// Compare all licenses key with keyword from: 
// https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/licensing-a-repository

fn calc_license(license_key: String) -> bool {
    // GitHub licenses must comply with lesser GNU v2.1
    let github_license = vec!["mit", "apache-2.0", "bsd-3-clause"]; // GitHub License List

    // Loop through and compare license key with GitHub license key word
    for lincense_keyword in github_license {
        if license_key == lincense_keyword { // There is a license, return 1
            return 1;
        } 
        
        return 0; // Else return 0
    }
}


