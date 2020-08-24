use reqwest::header::{ ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use dotenv::dotenv;
use reqwest::Error;
use reqwest::Client;
use std::collections::HashMap;
use std::env;

pub async fn make_github_repo(filename: &str) -> Result<(), Error> {
    dotenv().expect("Failed to set env");
    let token = env::var("GITHUB_ACCESS_TOKEN").unwrap();
    let mut map = HashMap::new();
    map.insert("name", filename);
    let client = Client::new();
    let body = client.post("https://api.github.com/users/repos/")
        .header(ACCEPT, "application/vnd.github.v3+json")
        .header(AUTHORIZATION, format!("token {}", token))
        .json(&map)
        .send()
        .await?;
    println!("{:?}", body);
    Ok(())
}
