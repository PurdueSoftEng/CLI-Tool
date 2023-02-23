#[tokio::main]
async fn main() {
    println!("{:?}", cli::working());
    let res = cli::rate("https://github.com/ljharb/qs", env!("GITHUB_TOKEN"))
        .await
        .unwrap();
    println!("{:?}", res);
    println!("{:?}", res.responsive());
    println!("Hello, world!");
}
