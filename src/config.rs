pub struct Config {
    api_key: String,
    model: String,
    base_url: String,
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
}

pub fn load_config(private: bool) -> Config {
    println!("Loading Config!");

    dotenvy::dotenv().ok();

    let mut preappend = "";
    if private {
        preappend = "PRIVATE_";
        println!("Loading Private AI");
    }

    let api_key = std::env::var(format!("{}{}", preappend, "API_KEY")).expect("API_KEY MISSING");
    let model = std::env::var(format!("{}{}", preappend, "MODEL")).expect("MODEL MISSING");
    let base_url = std::env::var(format!("{}{}", preappend, "BASE_URL")).expect("BASE_URL MISSING");

    Config {
        api_key,
        model,
        base_url,
    }
}
