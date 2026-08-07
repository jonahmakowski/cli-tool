use super::ai_calls;
use anyhow::{Context, Result, bail};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn get_git_diff() -> Result<String, Box<dyn std::error::Error>> {
    let result = Command::new("git")
        .args(["--no-pager", "diff", "--staged"])
        .output()?;

    if !result.status.success() {
        return Err(format!(
            "git diff failed (exit code {}): {}",
            result.status,
            String::from_utf8_lossy(&result.stderr)
        )
        .into());
    }

    let diff = String::from_utf8(result.stdout)?;

    Ok(diff)
}

fn write_commit_message(
    config: &crate::config::Config,
    private_mode: bool,
    breaking: bool,
    intent: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let diff = get_git_diff()?;

    if diff.is_empty() {
        return Err("Looks like nothing's staged! Maybe run `git add`?".into());
    }

    let constructed_message = format!(
        "{}\n\nBreaking Change: {}\n\nIntent: {}",
        diff, breaking, intent
    );

    let ai_response = ai_calls::use_pattern(
        "git-diff-commit",
        &constructed_message,
        config,
        private_mode,
    )?;

    Ok(ai_response)
}

pub fn git_commit(
    config: &crate::config::Config,
    private_mode: bool,
    skip_preflight: bool,
    breaking: bool,
    intent: &str,
) {
    if !skip_preflight && *super::repo_functions::REPO_CONFIG_EXISTS {
        super::repo_functions::run_preflight().expect("Preflight failed");
    }

    match write_commit_message(config, private_mode, breaking, intent) {
        Ok(response) => {
            let dirs = ProjectDirs::from("com", "jonahmakowski", "cli-tool").unwrap();
            let cache_dir = dirs.cache_dir();
            let cache_file = cache_dir.join("commit_msg.txt");

            fs::create_dir_all(cache_dir).expect("FAILED TO CREATE CACHE FOLDER");
            fs::write(&cache_file, response)
                .expect("FAILED TO WRITE TO `commit_msg.txt` IN CACHE FOLDER");

            let mut git_commit_command = Command::new("git")
                .args(["commit", "--edit", "--file", &cache_file.to_string_lossy()])
                .spawn()
                .expect("Failed to start git commit command");

            git_commit_command
                .wait()
                .expect("Git commit command failed");
        }
        Err(err) => eprintln!("Failed: {}", err),
    }
}

pub fn git_repo_root() -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .context("failed to run git")?;

    if !output.status.success() {
        bail!("not inside a Git working tree");
    }

    let root =
        String::from_utf8(output.stdout).context("git returned a non-UTF-8 repository path")?;

    Ok(PathBuf::from(root.trim()))
}
