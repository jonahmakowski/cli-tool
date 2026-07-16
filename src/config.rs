pub struct Config {
    api_key: String,
    model: String,
    base_url: String,
    prompt_dir: String,
}

impl Config {
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn prompt_dir(&self) -> &str {
        &self.prompt_dir
    }
}

pub fn load_config() -> Config {
    println!("Loading Config!");

    dotenvy::dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY MISSING");
    let model = std::env::var("MODEL").expect("MODEL MISSING");
    let base_url = std::env::var("BASE_URL").expect("BASE_URL MISSING");
    Config {
        api_key,
        model,
        base_url,
        prompt_dir: "./prompts".to_string(),
    }
}
