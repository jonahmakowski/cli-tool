use super::ai_calls;
use std::fs;
use std::process::{Command, Stdio};

fn get_subtitles(url: &str, show_logs: bool) -> Result<String, Box<dyn std::error::Error>> {
    let fixed_url = url.replace("invidious.jonahmakowski.ca", "youtube.com");

    println!("Loading Subtitles");

    let tempdir = tempfile::tempdir().unwrap();
    let subs_path = tempdir.path().join("subs");

    let mut command = Command::new("yt-dlp");

    command
        .args([
            "--write-auto-subs",
            "--sub-langs",
            "en",
            "--sub-format",
            "vtt",
            "--skip-download",
            "-o",
        ])
        .arg(subs_path.to_str().unwrap())
        .arg(fixed_url);

    if !show_logs {
        command.stdout(Stdio::null()).stderr(Stdio::null());
    }

    let status = command.status()?;

    if !status.success() {
        return Err("yt-dlp failed".into());
    }

    let vtt_path = tempdir.path().join("subs.en.vtt");
    let result = fs::read_to_string(vtt_path)?;
    println!("Subtitles length: {} characters", result.len());

    Ok(result)
}

pub fn run_summarize_yt(config: &crate::config::Config, url: &str, private_mode: bool) {
    let get_subs = get_subtitles(url, false);

    match get_subs {
        Ok(subtitles) => {
            match ai_calls::use_pattern("yt-summary", &subtitles, config, private_mode) {
                Ok(result) => {
                    println!("Summary:");
                    println!("{result}");
                }
                Err(err) => {
                    println!("Error: {}", err);
                }
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

pub fn download_yt(url: &str, target_location: &str) -> Result<(), Box<dyn std::error::Error>> {
    let fixed_url = url.replace("invidious.jonahmakowski.ca", "youtube.com");

    let mut command = Command::new("yt-dlp");

    command.args(["-t", "mp4"]);

    if !target_location.is_empty() {
        command.arg("--output").arg(target_location);
    }

    command.arg(fixed_url);

    let status = command.status()?;

    if !status.success() {
        return Err("yt-dlp failed".into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn subtitles_check() {
        let yt_subtitles = get_subtitles("https://www.youtube.com/watch?v=jObOjhUkf50", false)
            .expect("Failed to download subtitles from youtube");
        let invidious_subtitles = get_subtitles(
            "https://invidious.jonahmakowski.ca/watch?v=jObOjhUkf50",
            false,
        )
        .expect("Failed to download subtitles from invidious (but via youtube)");

        let correct_subtitles = include_str!("../../tests-data/subtitles_test.txt");

        assert_eq!(yt_subtitles, correct_subtitles);
        assert_eq!(invidious_subtitles, correct_subtitles);
    }
}
