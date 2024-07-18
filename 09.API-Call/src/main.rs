use reqwest::{header::USER_AGENT, Error};
use serde::Deserialize;


#[derive(Deserialize,Debug)]
struct User{
    login: String,
    id: u32
}



#[tokio::main]
async fn main() -> Result<(),Error>{

    // 1.Set the api_url you need to request

    let result_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers"
        ,owner = "York-lap"
        ,repo = "Rust-Practice-Project");

    println!("{}",result_url);

    // 2.Set client to send the request
    
    let client = reqwest::Client::new();

    let response = client
        .get(&result_url)
        .header(USER_AGENT, "rustWeb-api-requst-demo")
        .send()
        .await?;

    // 3.Set users to recept the client's response
    
    let users:Vec<User> = response.json().await?;

    println!("{:#?}",users);

    Ok(())

}