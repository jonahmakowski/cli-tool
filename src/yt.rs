use std::fs;
use std::process::Command;

pub fn get_subtitles(url: &str) -> Result<String, Box<dyn std::error::Error>> {
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
        .status()?;

    if !status.success() {
        return Err("yt-dlp failed".into())
    }

    let vtt_path = tempdir.path().join("subs.en.vtt");
    let result = fs::read_to_string(vtt_path)?;
    println!("Subtitles length: {} characters", result.len());

    Ok(result)
}
