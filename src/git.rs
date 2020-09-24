use dotenv::dotenv;
use reqwest::header::USER_AGENT;
use reqwest::Client;
use reqwest::Error;
use std::collections::HashMap;
use std::env;
use std::process::Command;

pub async fn make_github_repo(filename: &str) -> Result<(), Error> {
    //setting dotenv variables to env variables
    /*dotenv().expect("Failed to set env");
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
    */
    let cd_output = Command::new("git")
        .arg("init")
        .arg(&filename)
        .output()
        .expect("Could not process cd command");

    let output = match String::from_utf8(cd_output.stdout) {
        Ok(y) => y,
        Err(e) => e.to_string(),
    };
    println!(
        "\n{} Type the following commands to get started!\ncd newproject\ngit remote add origin",
        output
    );
    Ok(())
}

pub fn addCommitPush(branch: &str, commit_message: &str) -> Result<(), ()> {
    let git_commit = Command::new("git")
        .arg("commit")
        .arg("-a")
        .arg(format!("-m{}", &commit_message))
        .output()
        .expect("Could not push changes");

    let output = match String::from_utf8(git_commit.stdout) {
        Ok(y) => y,
        Err(e) => e.to_string(),
    };


    println!(
        "\n{}",
        output
    );

    let git_push = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .output()
        .expect("Could not push changes");

    let push_output = match String::from_utf8(git_push.stdout) {
        Ok(y) => y,
        Err(e) => e.to_string(),
    };
    println!(
        "\n{}",
        push_output
    );

    Ok(())
}
