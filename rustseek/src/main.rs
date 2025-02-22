use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::io::{self, Write};
use std::sync::OnceLock;

static APIURL: OnceLock<String> = OnceLock::new();
static MODEL_NAME: OnceLock<String> = OnceLock::new();

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

async fn ollama_query(prompt: &str) -> Result<String, Box<dyn Error>> {
    let model_name = MODEL_NAME.get().expect("Model name not set");
    let client = Client::new();
    let request = OllamaRequest {
        model: model_name.clone(),
        prompt: prompt.to_string(),
        stream: false,
    };

    let api_url = APIURL.get().expect("API URL not set");
    let res: OllamaResponse = client
        .post(api_url)
        .json(&request)
        .send()
        .await?
        .json()
        .await?;

    Ok(res.response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let stdin = io::stdin();

    if args.len() < 3 {
        println!("Usage: {} <model> <APIServer>", args[0]);
        return Ok(());
    }
    MODEL_NAME
        .set(args[1].clone())
        .expect("Model name already set");
    APIURL.set(args[2].clone()).expect("API URL already set");
    loop {
        let mut prompt = String::new();
        print!("Ask LLM: (xit to exit) ");
        io::stdout().flush().expect("Failed to flush stdout");

        stdin.read_line(&mut prompt).unwrap();

        let prompt = prompt.trim();

        if prompt.eq_ignore_ascii_case("xit") {
            break;
        }

        let response = ollama_query(prompt).await?;
        println!("{}", response);
    }
    Ok(())
}
