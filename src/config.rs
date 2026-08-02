use directories::ProjectDirs;
use getset::Getters;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct Config {
    pub ai: AiConfig,
    pub tv: TvConfig,
}

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct AiConfig {
    pub private: AiConfigChild,
    pub public: AiConfigChild,
}

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct AiConfigChild {
    api_key: String,
    model: String,
    base_url: String,
}

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct TvConfig {
    api_key: Option<String>,
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
