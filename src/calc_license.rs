
// Rust function to get license metric result
// If a the license key matches a keyword from 
// the compatible GitHub licenses, print 1, else print 0 
// Compare all licenses key with keyword from: 
// https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/licensing-a-repository

pub async fn calc_licenses(license_key: String) -> i8 {
    // GitHub licenses must comply with lesser GNU v2.1

    if is_valid(license_key)
    {
        return 1
    }
    return 0
}

fn is_valid(license_key: String) -> bool
{
    let github_license = vec![String::from("mit"), String::from("apache-2.0"), String::from("bsd-3-clause")];
    (github_license.contains(&license_key))
}


