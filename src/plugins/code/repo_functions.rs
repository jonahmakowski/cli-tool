use super::git;
use anyhow::Result;
use indexmap::IndexMap;
use serde::Deserialize;
use std::process::Command;
use std::{fs, sync::LazyLock};
use thiserror::Error;

const CONFIG_FILE: &str = "tool.yaml";

static REPO_CONFIG: LazyLock<Result<RepoConfig, ConfigError>> = LazyLock::new(load_config);
pub static REPO_CONFIG_EXISTS: LazyLock<bool> = LazyLock::new(|| REPO_CONFIG.is_ok());

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("the configuration file doesn't exist")]
    NoFile,

    #[error("this is not a git repo: {source}")]
    NoGit {
        #[source]
        source: anyhow::Error,
    },

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

#[derive(Debug, Error)]
pub enum RepoFuncError {
    #[error("this is not a git repo")]
    NoGit,
    #[error("config is not correct: {source}")]
    ConfigError {
        #[source]
        source: &'static ConfigError,
    },
    #[error("required arguements not provided. This function requires: {:?}", args)]
    RequiredArgumentsNotProvided { args: Vec<String> },
    #[error("config is not correct: {information}")]
    Custom { information: String },
}

#[derive(Debug, Deserialize, PartialEq)]
struct RepoConfig {
    preflight_commands: Option<Vec<LintCommandConfig>>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct LintCommandConfig {
    name: String,
    command: Vec<String>,
}

fn load_config() -> Result<RepoConfig, ConfigError> {
    match git::git_repo_root() {
        Ok(path) => {
            let file_path = path.join(CONFIG_FILE);

            if !file_path.exists() {
                return Err(ConfigError::NoFile);
            }

            match fs::read_to_string(&file_path) {
                Ok(data) => {
                    let config_error: Result<RepoConfig, yaml_serde::Error> =
                        yaml_serde::from_str(&data);

                    match config_error {
                        Ok(config) => Ok(config),
                        Err(err) => Err(ConfigError::Parse {
                            path: file_path,
                            source: err,
                        }),
                    }
                }
                Err(err) => Err(ConfigError::Read {
                    path: file_path,
                    source: err,
                }),
            }
        }
        Err(err) => Err(ConfigError::NoGit { source: err }),
    }
}

pub fn run_preflight() -> Result<(), RepoFuncError> {
    let conf = &(*REPO_CONFIG);

    match conf {
        Ok(conf) => match &conf.preflight_commands {
            Some(commands) => {
                if commands.is_empty() {
                    return Err(RepoFuncError::RequiredArgumentsNotProvided {
                        args: vec!["preflight_commands.*".to_string()],
                    });
                }

                let mut results = IndexMap::new();
                let mut failed = false;

                for command in commands {
                    println!("------------------ {} ------------------", command.name);

                    if command.command.is_empty() {
                        return Err(RepoFuncError::RequiredArgumentsNotProvided {
                            args: vec!["preflight_commands.*.command".to_string()],
                        });
                    }

                    let mut cmd = Command::new(&command.command[0]);

                    command
                        .command
                        .iter()
                        .enumerate()
                        .for_each(|(index, part)| {
                            if index != 0 {
                                cmd.arg(part);
                            }
                        });

                    let out = cmd.status();

                    match out {
                        Ok(status) => {
                            if status.success() {
                                results.insert(&command.name, true);
                            } else {
                                results.insert(&command.name, false);
                                failed = true;
                            }
                        }
                        Err(_) => {
                            results.insert(&command.name, false);
                            failed = true;
                        }
                    }
                }

                println!("------------------ Overview ------------------");
                for (name, result) in results {
                    if result {
                        println!("✓ {}", name)
                    } else {
                        println!("✖ {}", name)
                    }
                }

                if failed {
                    return Err(RepoFuncError::Custom {
                        information: "a command has failed".to_string(),
                    });
                }

                Ok(())
            }
            None => Err(RepoFuncError::RequiredArgumentsNotProvided {
                args: vec!["preflight_commands.*".to_string()],
            }),
        },
        Err(err) => match err {
            ConfigError::NoGit { .. } => Err(RepoFuncError::NoGit),
            _ => Err(RepoFuncError::ConfigError { source: err }),
        },
    }
}
