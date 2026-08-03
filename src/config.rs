use directories::ProjectDirs;
use serde::Deserialize;
use std::{fs, path::PathBuf};
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("could not determine the configuration directory")]
    NoProjectDirectory,

    #[error("could not read configuration file at {path}: {source}")]
    Read {
        path: std::path::PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("invalid YAML in configuration file at {path}: {source}")]
    Parse {
        path: std::path::PathBuf,
        #[source]
        source: yaml_serde::Error,
    },
}

fn default_path() -> Result<PathBuf, ConfigError> {
    match ProjectDirs::from("com", "jonahmakowski", "cli-tool") {
        Some(proj_dirs) => Ok(proj_dirs.config_dir().join("config.yaml")),
        None => Err(ConfigError::NoProjectDirectory),
    }
}

pub fn load_config(path: Option<PathBuf>) -> Result<Config, ConfigError> {
    let config_path = match path {
        Some(p) => p,
        None => default_path()?,
    };

    match fs::read_to_string(&config_path) {
        Ok(config_text) => match yaml_serde::from_str::<Config>(&config_text) {
            Ok(config) => Ok(config),
            Err(err) => Err(ConfigError::Parse {
                path: config_path,
                source: err,
            }),
        },
        Err(err) => Err(ConfigError::Read {
            path: config_path,
            source: err,
        }),
    }
}
