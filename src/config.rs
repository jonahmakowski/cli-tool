use directories::ProjectDirs;
use serde::Deserialize;
use std::{fs, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub ai: AiConfig,
    pub tv: Option<TvConfig>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AiConfig {
    pub private: AiConfigChild,
    pub public: AiConfigChild,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AiConfigChild {
    pub api_key: String,
    pub model: String,
    pub base_url: String,
}

#[derive(Debug, Deserialize, PartialEq)]
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

pub fn default_path() -> Result<PathBuf, ConfigError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::path::Path;
    use tempfile::tempdir;

    fn create_mock_config_file(
        dir: &Path,
        text: &str,
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let temp_file = dir.join("config.yaml");

        fs::write(&temp_file, text)?;

        Ok(temp_file)
    }

    #[test]
    fn valid_config() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;

        let config_valid = indoc! {"
            ai:
                private:
                    api_key: \"private_key\"
                    model: \"private_model\"
                    base_url: \"http://127.0.0.1:11434/v1\"
                public:
                    api_key: \"public_key\"
                    model: \"public_model\"
                    base_url: \"https://example.com/v1\"
            tv:
                api_key: \"tvdb_key\"
        "};

        let config_path = create_mock_config_file(temp_dir.path(), config_valid)?;

        assert_eq!(
            load_config(Some(config_path))?,
            Config {
                ai: AiConfig {
                    private: AiConfigChild {
                        api_key: "private_key".into(),
                        model: "private_model".into(),
                        base_url: "http://127.0.0.1:11434/v1".into(),
                    },
                    public: AiConfigChild {
                        api_key: "public_key".into(),
                        model: "public_model".into(),
                        base_url: "https://example.com/v1".into(),
                    }
                },
                tv: Some(TvConfig {
                    api_key: Some("tvdb_key".into()),
                })
            }
        );

        Ok(())
    }

    #[test]
    fn valid_config_no_tv() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;

        let config_valid = indoc! {"
            ai:
                private:
                    api_key: \"private_key\"
                    model: \"private_model\"
                    base_url: \"http://127.0.0.1:11434/v1\"
                public:
                    api_key: \"public_key\"
                    model: \"public_model\"
                    base_url: \"https://example.com/v1\"
        "};

        let config_path = create_mock_config_file(temp_dir.path(), config_valid)?;

        assert_eq!(
            load_config(Some(config_path))?,
            Config {
                ai: AiConfig {
                    private: AiConfigChild {
                        api_key: "private_key".into(),
                        model: "private_model".into(),
                        base_url: "http://127.0.0.1:11434/v1".into(),
                    },
                    public: AiConfigChild {
                        api_key: "public_key".into(),
                        model: "public_model".into(),
                        base_url: "https://example.com/v1".into(),
                    }
                },
                tv: None,
            }
        );

        Ok(())
    }
    #[test]
    fn invalid_config() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;

        let config_valid = indoc! {"
            ai:
                private:
                    api_key: \"private_key\"
                    model: \"private_model\"
                    base_url: \"http://127.0.0.1:11434/v1\"
                public:
                    api_key: \"public_key\"
                    model: \"public_model\"
            tv:
                api_key: \"tvdb_key\"
        "};

        let config_path = create_mock_config_file(temp_dir.path(), config_valid)?;

        match load_config(Some(config_path)) {
            Ok(_) => panic!(),
            Err(err) => {
                if let ConfigError::Parse { .. } = err {
                    return Ok(());
                }

                panic!()
            }
        }
    }
}
