use super::git;
use anyhow::Result;
use indexmap::IndexMap;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, sync::LazyLock};
use tempfile::{TempDir, tempdir};
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
    post_command: Option<Vec<String>>,
    #[serde(rename = "if")]
    condition: Option<String>,
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

fn execute_command_from_vec(
    command: &[String],
    execution_location: &Path,
) -> Result<std::process::ExitStatus, std::io::Error> {
    let mut cmd = Command::new(&command[0]);

    cmd.current_dir(execution_location);

    command.iter().enumerate().for_each(|(index, part)| {
        if index != 0 {
            cmd.arg(part);
        }
    });

    cmd.status()
}

fn execute_git_command(
    repository: &Path,
    args: &[&str],
) -> Result<std::process::Output, RepoFuncError> {
    Command::new("git")
        .current_dir(repository)
        .args(args)
        .output()
        .map_err(|err| RepoFuncError::Custom {
            information: format!("failed to run git: {err}"),
        })
}

struct WorktreeGuard {
    repository: PathBuf,
    worktree: PathBuf,
}

impl Drop for WorktreeGuard {
    fn drop(&mut self) {
        let _ = execute_git_command(
            &self.repository,
            &[
                "worktree",
                "remove",
                "--force",
                &self.worktree.to_string_lossy(),
            ],
        );
    }
}

fn create_staged_worktree(repository: &Path) -> Result<(TempDir, WorktreeGuard), RepoFuncError> {
    let worktree = tempdir().map_err(|err| RepoFuncError::Custom {
        information: format!("failed to create temporary directory: {err}"),
    })?;

    let worktree_path = worktree.path().to_path_buf();
    let worktree_path_string = worktree_path.to_string_lossy().into_owned();
    let add = execute_git_command(
        repository,
        &["worktree", "add", "--detach", &worktree_path_string, "HEAD"],
    )?;
    if !add.status.success() {
        return Err(RepoFuncError::Custom {
            information: format!(
                "failed to create temporary worktree: {}",
                String::from_utf8_lossy(&add.stderr).trim()
            ),
        });
    }

    let staged_diff = git::get_staged_diff_at(repository).map_err(|err| RepoFuncError::Custom {
        information: format!("failed to read staged diff: {err}"),
    })?;

    if !staged_diff.is_empty() {
        let apply = Command::new("git")
            .current_dir(&worktree_path)
            .args(["apply", "--index"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                use std::io::Write;

                child
                    .stdin
                    .take()
                    .expect("git apply stdin should be piped")
                    .write_all(&staged_diff)?;
                child.wait_with_output()
            })
            .map_err(|err| RepoFuncError::Custom {
                information: format!("failed to apply staged diff: {err}"),
            })?;

        if !apply.status.success() {
            return Err(RepoFuncError::Custom {
                information: format!(
                    "failed to apply staged diff: {}",
                    String::from_utf8_lossy(&apply.stderr).trim()
                ),
            });
        }
    }

    Ok((
        worktree,
        WorktreeGuard {
            repository: repository.to_path_buf(),
            worktree: worktree_path,
        },
    ))
}

fn check_conditional(conditional: &str, repository: &Path) -> Result<bool, RepoFuncError> {
    let conditional_vec: Vec<&str> = conditional.split(" ").collect();

    match conditional_vec[0] {
        "changed" => {
            if conditional_vec.len() == 1 {
                return Err(RepoFuncError::RequiredArgumentsNotProvided {
                    args: vec!["preflight_commands.*.if.+1".into()],
                });
            }

            let git_diff = match git::get_staged_diff_at(repository) {
                Ok(val) => String::from_utf8(val).map_err(|_| RepoFuncError::Custom {
                    information: "staged diff is not valid UTF-8".into(),
                })?,
                Err(_) => return Err(RepoFuncError::NoGit),
            };

            let git_diff_lines = git_diff.split("\n");

            let mut changed_files: Vec<&str> = vec![];

            for line in git_diff_lines {
                if line.starts_with("diff --git") {
                    let whitespace_split_line: Vec<&str> = line.split(" ").collect();
                    changed_files
                        .push(whitespace_split_line[2].split("a/").collect::<Vec<&str>>()[1]);
                }
            }

            for (index, val) in conditional_vec.iter().enumerate() {
                if index == 0 {
                    continue;
                }

                if changed_files.contains(val) {
                    return Ok(true);
                }
            }

            Ok(false)
        }
        _ => Err(RepoFuncError::Custom {
            information: "Invalid conditional arguement".into(),
        }),
    }
}

pub fn run_preflight() -> Result<(), RepoFuncError> {
    let conf = &(*REPO_CONFIG);

    match conf {
        Ok(conf) => match &conf.preflight_commands {
            Some(commands) => {
                let git_repo_path = git::git_repo_root().map_err(|_| RepoFuncError::NoGit)?;

                if commands.is_empty() {
                    return Err(RepoFuncError::RequiredArgumentsNotProvided {
                        args: vec!["preflight_commands.*".to_string()],
                    });
                }

                let (staged_worktree, _worktree_guard) = create_staged_worktree(&git_repo_path)?;
                let execution_location = staged_worktree.path();
                let mut results = IndexMap::new();
                let mut failed = false;

                for command in commands {
                    println!("------------------ {} ------------------", command.name);

                    if let Some(cond) = &command.condition
                        && !check_conditional(cond, &git_repo_path)?
                    {
                        println!("Skipped due to conditions");
                        results.insert(&command.name, true);
                        continue;
                    }

                    if command.command.is_empty() {
                        return Err(RepoFuncError::RequiredArgumentsNotProvided {
                            args: vec!["preflight_commands.*.command".to_string()],
                        });
                    }

                    let out = execute_command_from_vec(&command.command, execution_location);

                    match out {
                        Ok(status) => {
                            if status.success() {
                                if let Some(cmd) = &command.post_command {
                                    let out = execute_command_from_vec(cmd, execution_location);
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
                                } else {
                                    results.insert(&command.name, true);
                                }
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
