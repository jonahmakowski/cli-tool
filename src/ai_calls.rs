use crate::config;
use reqwest::blocking::Client;
use serde_json::json;
use std::collections::HashMap;
use std::sync::LazyLock;

static PATTERNS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        ("yt-summary", include_str!("../prompts/yt-summary.md")),
        (
            "git-diff-commit",
            include_str!("../prompts/git-diff-commit.md"),
        ),
    ])
});

pub fn base_call(
    system_prompt: &str,
    user_message: &str,
    config: &config::Config,
    private_mode: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let ai_config = {
        if private_mode {
            &config.ai.private
        } else {
            &config.ai.public
        }
    };

    let response = client
        .post(format!("{}/chat/completions", ai_config.base_url()))
        .bearer_auth(ai_config.api_key())
        .json(&json!({
            "model": ai_config.model(),
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
    private_mode: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    if !PATTERNS.contains_key(pattern) {
        return Err("Patern doesn't exist".into());
    }

    let system_prompt = PATTERNS[pattern];

    base_call(system_prompt, user_message, config, private_mode)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread;

    fn config(base_url: &str) -> config::Config {
        let yaml = format!(
            r#"
ai:
  private:
    api_key: private-key
    model: private-model
    base_url: {base_url}
  public:
    api_key: public-key
    model: public-model
    base_url: {base_url}
tv: {{}}
"#
        );
        yaml_serde::from_str(&yaml).unwrap()
    }

    fn mock_server(response: &'static str) -> (String, thread::JoinHandle<String>) {
        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let address = format!("http://{}", listener.local_addr().unwrap());
        let handle = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut bytes = Vec::new();
            let mut buffer = [0; 1024];
            let body_length = loop {
                let count = stream.read(&mut buffer).unwrap();
                assert!(count > 0);
                bytes.extend_from_slice(&buffer[..count]);

                if let Some(headers_end) = bytes.windows(4).position(|window| window == b"\r\n\r\n")
                {
                    let headers = String::from_utf8_lossy(&bytes[..headers_end]);
                    break headers
                        .lines()
                        .find_map(|line| {
                            line.strip_prefix("Content-Length: ")
                                .and_then(|value| value.parse::<usize>().ok())
                        })
                        .unwrap_or(0);
                }
            };

            let headers_end = bytes
                .windows(4)
                .position(|window| window == b"\r\n\r\n")
                .unwrap()
                + 4;
            while bytes.len() < headers_end + body_length {
                let count = stream.read(&mut buffer).unwrap();
                assert!(count > 0);
                bytes.extend_from_slice(&buffer[..count]);
            }

            let request = String::from_utf8(bytes).unwrap();
            stream
                .write_all(
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        response.len(), response
                    )
                    .as_bytes(),
                )
                .unwrap();
            request
        });
        (address, handle)
    }

    #[test]
    fn rejects_unknown_pattern_without_an_http_request() {
        let result = use_pattern("missing", "message", &config("http://127.0.0.1:1"), false);

        assert_eq!(result.unwrap_err().to_string(), "Patern doesn't exist");
    }

    #[test]
    fn known_pattern_uses_the_bundled_prompt() {
        assert!(PATTERNS.contains_key("yt-summary"));
        assert!(!PATTERNS["yt-summary"].trim().is_empty());
    }

    #[test]
    fn base_call_uses_public_credentials_and_serializes_messages() {
        let (base_url, server) =
            mock_server(r#"{"choices":[{"message":{"content":"public answer"}}]}"#);
        let result = base_call("system text", "user text", &config(&base_url), false).unwrap();
        let request = server.join().unwrap();

        assert_eq!(result, "public answer");
        assert!(request.contains("public-key"), "request was: {request}");
        assert!(request.contains("\"model\":\"public-model\""));
        assert!(request.contains("\"role\":\"system\""));
        assert!(request.contains("system text"));
        assert!(request.contains("user text"));
    }

    #[test]
    fn base_call_uses_private_credentials_when_requested() {
        let (base_url, server) =
            mock_server(r#"{"choices":[{"message":{"content":"private answer"}}]}"#);
        let result = base_call("system", "user", &config(&base_url), true).unwrap();
        let request = server.join().unwrap();

        assert_eq!(result, "private answer");
        assert!(request.contains("private-key"), "request was: {request}");
        assert!(request.contains("\"model\":\"private-model\""));
    }

    #[test]
    fn base_call_rejects_responses_without_content() {
        let (base_url, server) = mock_server(r#"{"choices":[]}"#);

        let result = base_call("system", "user", &config(&base_url), false);
        server.join().unwrap();

        assert_eq!(result.unwrap_err().to_string(), "No response content");
    }
}
