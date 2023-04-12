use reqwest;
use tokio;
use ::serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Request {
    prompt: String,
    length: u8,
}

#[derive(Deserialize, Debug)]
struct Response {
    text: String,
}

// Create constant in rust
const API_KEY: &str = "YOUR_API_KEY";

async fn chat_gpt_request(input: &str) -> Result<String, reqwest::Error> {
    let request = Request {
        prompt: input.to_string(),
        length: 50,
    };
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/engines/davinci-codex/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", API_KEY))
        .json(&request)
        .send()
        .await?
        .json::<Response>()
        .await?;
    Ok(response.text)
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    chat_gpt_request("Hello, world!").await.unwrap();
}