use directories::ProjectDirs;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub ai: AiConfig,
    pub tv: TvConfig,
}

#[derive(Debug, Deserialize)]
pub struct AiConfig {
    pub private: AiConfigChild,
    pub public: AiConfigChild,
}

#[derive(Debug, Deserialize)]
pub struct AiConfigChild {
    pub api_key: String,
    pub model: String,
    pub base_url: String,
}

#[derive(Debug, Deserialize)]
pub struct TvConfig {
    pub api_key: Option<String>,
}

pub fn load_config() -> Config {
    match ProjectDirs::from("com", "jonahmakowski", "cli-tool") {
        Some(proj_dirs) => {
            let config_path = proj_dirs.config_dir().join("config.yaml");

            match fs::read_to_string(&config_path) {
                Ok(config_text) => match yaml_serde::from_str::<Config>(&config_text) {
                    Ok(config) => config,
                    Err(_) => panic!("Config file has syntax error"),
                },
                Err(_) => panic!(
                    "Could not read config file. Maybe it doesn't exist? Create it at \"{}\"",
                    config_path.to_string_lossy()
                ),
            }
        }
        None => panic!("Couldn't find a valid location for config file"),
    }
}
