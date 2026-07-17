use super::ai_calls;
use std::fs;
use std::process::{Command, Stdio};

fn get_subtitles(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("Loading Subtitles");

    let tempdir = tempfile::tempdir().unwrap();
    let subs_path = tempdir.path().join("subs");

    let status = Command::new("yt-dlp")
        .args([
            "--write-auto-subs",
            "--sub-langs", "en",
            "--sub-format", "vtt",
            "--skip-download",
            "-o",
        ])
        .arg(subs_path.to_str().unwrap())
        .arg(url)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    if !status.success() {
        return Err("yt-dlp failed".into())
    }

    let vtt_path = tempdir.path().join("subs.en.vtt");
    let result = fs::read_to_string(vtt_path)?;
    println!("Subtitles length: {} characters", result.len());

    Ok(result)
}


pub fn run_summarize_yt(config: &crate::config::Config, url: &str) -> () {
    let fixed_url = url.replace("invidious.jonahmakowski.ca", "youtube.com");

    let get_subs = get_subtitles(&fixed_url);

    match get_subs {
        Ok(subtitles) => {
            match ai_calls::use_pattern("yt-summary", &subtitles, &config) {
                Ok(result) => {
                    println!("Summary:");
                    println!("{result}");
                    return ()
                }
                Err(err) => {
                    println!("Error: {}", err);
                    return ()
                }
            }
        }
        Err(err) => {
            println!("Error: {}", err);
            return ()
        }
    }
}
