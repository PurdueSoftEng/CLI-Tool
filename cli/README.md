# `cli`

The `cli` crate is modifies from project part 1 of another team.

Exposed functions:

- `async rate(url: &str, token: &str) -> Option<GithubRepo>` takes a url and returns the calculated scores.

To get the individual scores out of `GithubRepo`, use one of the following: `overall()`, `bus()`, `correct()`, `license()`, `responsive()`, `rampup()`, `version()`, `review()`

`GithubRepo` implements debug.

## Example usage

```
#[tokio::main]
async fn main() {
    let res = cli::rate("https://github.com/postcss/postcss", env!("GITHUB_TOKEN"))
        .await
        .unwrap();
    println!("{:?}", res);
    println!("{:?}", res.version());
}
```

---

original readme

# CLI-Tool
A command line interface tool to assess potential packages for open source development.

A ECE 461 Project to create a CLI .

Gabby Whitis

Colleen Granelli

Isaac Hagedorn

Emma Misenheimer
