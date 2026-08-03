mod ai_calls;
mod config;
mod plugins;
use clap::{Parser, Subcommand};
use reqwest::blocking::Client;
use std::sync::LazyLock;

static WEB_CLIENT: LazyLock<Client> = LazyLock::new(Client::new);

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(
        short = 'p',
        long,
        global = true,
        help = "Use the private AI configuration in relevant workflows"
    )]
    private: bool,
}

#[derive(Subcommand)]
enum Command {
    /// Summarize content from various sources
    Summarize {
        #[command(subcommand)]
        target: SummarizeTarget,
    },
    /// Get data from the internet
    Net {
        #[command(subcommand)]
        target: NetTarget,
    },
    /// Run commands that modify or are useful for code
    Code {
        #[command(subcommand)]
        target: CodeTarget,
    },
    /// Run diagnostics on your settings
    Doctor,
    /// Get information about your config
    Config {
        #[command(subcommand)]
        target: ConfigTarget,
    },
}

#[derive(Subcommand)]
enum SummarizeTarget {
    #[command(about = "Summarize content from youtube, provided a youtube or invidious url")]
    Yt {
        /// Url of the youtube video
        url: String,
    },
}

#[derive(Subcommand)]
enum NetTarget {
    /// Get Weather Data
    Weather,
    /// Get your public ip address
    Ip {
        /// Get IpV6 information
        #[arg(short = '6')]
        ip_v6: bool,
    },
    /// Download data from various sources
    Download {
        #[command(subcommand)]
        target: NetDownloadTarget,
    },
    /// Search various sources
    Search {
        #[command(subcommand)]
        target: SearchTarget,
    },
    /// Get the contents of a webpage
    Fetch {
        /// URL you want to fetch
        url: String,
        /// Get your result in markdown format (or html by default)
        #[arg(long, short = 'm')]
        markdown: bool,
    },
}

#[derive(Subcommand)]
enum NetDownloadTarget {
    /// Download a video from youtube or other yt-dlp supported site
    Yt {
        /// Url of the youtube video
        url: String,
        /// Name for the output, default
        #[arg(short = 'o', long, default_value = "")]
        output: String,
    },
}

#[derive(Subcommand)]
enum SearchTarget {
    /// Search for something on the tvdb
    Tvdb {
        /// What you want to search for
        parameter: String,
        /// Maximum number of results, defaults to 10
        #[arg(default_value_t = 10)]
        max: i64,
    },
}

#[derive(Subcommand)]
enum CodeTarget {
    /// Write a git commit message
    #[command(name = "git_commit")]
    GitCommit {
        /// This is a breaking change
        #[arg(long, default_value_t = false)]
        breaking: bool,
        /// What was your goal with this commit
        #[arg(default_value_t = String::new())]
        intent: String,
    },
}

#[derive(Subcommand)]
enum ConfigTarget {
    /// Returns the default config path
    Path,
    /// Displays the current config
    Show,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Command::Doctor = cli.command {
        plugins::doctor::run_checks_graphic();
        return Ok(());
    }

    let config = config::load_config(None)?;

    match cli.command {
        Command::Summarize { target } => match target {
            SummarizeTarget::Yt { url } => {
                plugins::yt::run_summarize_yt(&config, &url, cli.private);
            }
        },
        Command::Net { target } => match target {
            NetTarget::Weather => {
                plugins::net::get_weather_data();
            }
            NetTarget::Ip { ip_v6 } => {
                let ip = plugins::net::get_public_ip({
                    if ip_v6 {
                        &plugins::net::IpType::V6
                    } else {
                        &plugins::net::IpType::V4
                    }
                });

                match ip {
                    Ok(data) => println!("Public IP Address: {}", data),
                    Err(err) => eprintln!("Error: {}", err),
                }
            }
            NetTarget::Download { target } => match target {
                NetDownloadTarget::Yt { url, output } => {
                    match plugins::yt::download_yt(&url, &output) {
                        Ok(_) => println!("Downloaded successfully"),
                        Err(err) => eprintln!("Error: {}", err),
                    }
                }
            },
            NetTarget::Search { target } => match target {
                SearchTarget::Tvdb { parameter, max } => {
                    plugins::tv::tui_search(
                        &{
                            match config.tv {
                                Some(data) => data.api_key,
                                None => None,
                            }
                        },
                        &parameter,
                        max,
                    );
                }
            },
            NetTarget::Fetch { url, markdown } => {
                if markdown {
                    println!("{}", plugins::net::fetch_as_markdown(&url)?);
                } else {
                    println!("{}", plugins::net::fetch(&url)?);
                }
            }
        },
        Command::Code { target } => match target {
            CodeTarget::GitCommit { breaking, intent } => {
                plugins::git::git_commit(&config, cli.private, breaking, &intent);
            }
        },
        Command::Doctor => {}
        Command::Config { target } => match target {
            ConfigTarget::Path => println!("{}", config::default_path()?.to_string_lossy()),
            ConfigTarget::Show => println!("{:#?}", &config),
        },
    }

    Ok(())
}
