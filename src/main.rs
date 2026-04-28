use clap::Parser;
use serde::{Deserialize, Serialize};
use std::env;
use reqwest::Client;

#[derive(Parser, Debug)]
#[command(author, version, about = "A cowsay application that can use an LLM to generate messages")]
struct Args {
    /// The message to say
    message: Option<String>,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct Choice {
    message: ChatMessage,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

fn render_cow(message: &str) {
    let words: Vec<&str> = message.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in words {
        if current_line.len() + word.len() + 1 <= 40 {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        } else {
            lines.push(current_line.clone());
            current_line = word.to_string();
        }
    }
    lines.push(current_line);

    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    
    println!("  _{}", "_".repeat(max_len + 2));
    for line in &lines {
        println!(" / {:<width$} \\", line, width = max_len + 2);
    }
    println!("  {}{} ", " ".repeat(max_len), "_");
    println!("       \\   ^__^");
    println!("       \\   (oo)\\_______");
    println!("           (__)\\       \\\\");
    println!("               ||  ||    \\\\");
    println!("               ||  ||     \\\\");
}

async fn fetch_llm_message() -> Result<String, String> {
    let base_url = env::var("LLM_BASE_URL").map_err(|_| "LLM_BASE_URL env var not set".to_string())?;
    let model = env::var("LLM_MODEL").map_err(|_| "LLM_MODEL env var not set".to_string())?;
    let api_key = env::var("LLM_API_KEY").map_err(|_| "LLM_API_KEY env var not set".to_string())?;

    let client = Client::new();
    let url = format!("{}/v1/chat/completions", base_url.trim_end_matches('/'));

    let request = ChatRequest {
        model,
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: "Say something brief, witty, and random. Keep it under 20 words.".to_string(),
        }],
    };

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_else(|_| "No error body".to_string());
        return Err(format!("API error ({}): {}", status, text));
    }

    let chat_response: ChatResponse = response.json().await.map_err(|e| format!("JSON parsing failed: {}", e))?;
    
    chat_response.choices.get(0)
        .map(|c| c.message.content.clone())
        .ok_or_else(|| "No choices returned from LLM".to_string())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let message = match args.message {
        Some(msg) => msg,
        None => {
            match fetch_llm_message().await {
                Ok(msg) => msg,
                Err(e) => {
                    // If the error mentions the env vars are missing, we follow the "nothing to say" fallback
                    if e.contains("env var not set") {
                        "There is nothing to say.".to_string()
                    } else {
                        format!("Error: {}", e)
                    }
                }
            }
        }
    };

    render_cow(&message);
}
