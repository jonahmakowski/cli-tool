use crate::config;
use reqwest::blocking::Client;
use serde_json::json;
use std::collections::HashMap;
use std::sync::LazyLock;

static PATTERNS: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| HashMap::from([("yt-summary", include_str!("../prompts/yt-summary.md"))]));

pub fn base_call(
    system_prompt: &str,
    user_message: &str,
    config: &config::Config,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("Asking AI");

    let client = Client::new();

    let response = client
        .post(format!("{}/chat/completions", config.base_url()))
        .bearer_auth(config.api_key())
        .json(&json!({
            "model": config.model(),
            "messages": [
                {
                    "role": "system",
                    "content": system_prompt
                },
                {
                    "role": "user",
                    "content": user_message
                }
            ]
        }))
        .send()?;

    println!("Status: {}", response.status());

    let response_json: serde_json::Value = response.json()?;

    let answer = response_json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("No response content")?;

    Ok(answer.to_string())
}

pub fn use_pattern(
    pattern: &str,
    user_message: &str,
    config: &config::Config,
) -> Result<String, Box<dyn std::error::Error>> {
    if !PATTERNS.contains_key(pattern) {
        return Err("Patern doesn't exist".into());
    }

    let system_prompt = PATTERNS[pattern];

    base_call(&system_prompt, user_message, config)
}
