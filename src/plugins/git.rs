use super::ai_calls;
use std::process::Command;

fn get_git_diff() -> Result<String, Box<dyn std::error::Error>> {
    let result = Command::new("git").args(["--no-pager", "diff"]).output()?;

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

    println!("{}", ai_response);

    Ok("".into())
}

pub fn write_commit_message_wrapper(
    config: &crate::config::Config,
    private_mode: bool,
    breaking: bool,
    intent: &str,
) {
    match write_commit_message(config, private_mode, breaking, intent) {
        Ok(response) => println!("{}", response),
        Err(err) => eprintln!("Failed: {}", err),
    }
}
