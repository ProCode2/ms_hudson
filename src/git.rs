use reqwest::header::{ USER_AGENT };
use dotenv::dotenv;
use reqwest::Client;
use reqwest::Error;
use std::collections::HashMap;
use std::env;

pub async fn make_github_repo(filename: &str) -> Result<(), Error> {
    //setting dotenv variables to env variables
    dotenv().expect("Failed to set env");
    let token = env::var("GITHUB_ACCESS_TOKEN").unwrap();
    let username = env::var("GITHUB_USERNAME").unwrap();
    println!("{}:{}", username, token);

    let client = Client::new();

    let mut json_map = HashMap::new();
    json_map.insert("name", filename);

    let res = client.post("https://api.github.com/user/repos")
        .basic_auth(&username, Some(token))
        .header(USER_AGENT, &username)
        .json(&json_map)
        .send()
        .await?
        .text()
        .await?;
    println!("{:?}", res);
    Ok(())
}
