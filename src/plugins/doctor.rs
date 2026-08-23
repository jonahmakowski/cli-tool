use indexmap::IndexMap;
use std::process::Command;
use std::sync::LazyLock;

type CheckFn = fn() -> StatusCheck;

static CHECKS: LazyLock<IndexMap<&'static str, CheckFn>> = LazyLock::new(|| {
    IndexMap::from([
        ("Git Executable in PATH", check_git as CheckFn),
        ("YT DLP Executable in PATH", check_yt_dlp as CheckFn),
        ("rsync Executable in PATH", check_rsync as CheckFn),
        ("Configuration", validate_config as CheckFn),
    ])
});

enum StatusCheck {
    Passed(Option<String>),
    Warning(String),
    Failed(String),
}

impl StatusCheck {
    pub fn fancy_print(&self, check_name: &str) -> String {
        match self {
            Self::Passed(data) => match data {
                Some(text) => format!("✓ {} -- {}", check_name, text),
                None => format!("✓ {}", check_name),
            },
            Self::Warning(data) => format!("⚠ {} -- {}", check_name, data),
            Self::Failed(data) => format!("✖ {} -- {}", check_name, data),
        }
    }
}

fn check_external_dep(dep: &str) -> StatusCheck {
    match Command::new(dep).arg("--version").output() {
        Ok(result) => {
            if result.status.success() {
                let result_string = String::from_utf8_lossy(&result.stdout);

                StatusCheck::Passed(Some(format!("Version \"{}\"", result_string.trim())))
            } else {
                StatusCheck::Warning("Not found, related functions will not work".to_string())
            }
        }
        Err(err) => StatusCheck::Failed(format!("Failed to run command to check version: {}", err)),
    }
}

fn check_yt_dlp() -> StatusCheck {
    check_external_dep("yt-dlp")
}

fn check_git() -> StatusCheck {
    check_external_dep("git")
}

fn check_rsync() -> StatusCheck {
    check_external_dep("rsync")
}

fn validate_config() -> StatusCheck {
    match crate::config::load_config(None) {
        Ok(config) => {
            match config.tv {
                Some(key) => {
                    if key.api_key.is_none() {
                        return StatusCheck::Warning(
                            "TVDB api key is not set, related functions will not work".into(),
                        );
                    }
                }
                None => {
                    return StatusCheck::Warning(
                        "TV Section is not set, related functions will not work".into(),
                    );
                }
            }
            StatusCheck::Passed(None)
        }
        Err(err) => StatusCheck::Failed(err.to_string()),
    }
}

pub fn run_checks_graphic() {
    for (check_name, func) in CHECKS.iter() {
        let result = func();
        println!("{}", result.fancy_print(check_name));
    }
}
