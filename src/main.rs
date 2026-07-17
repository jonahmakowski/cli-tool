mod ai_calls;
mod config;
mod plugins;
use std::sync::LazyLock;
use clap::{Parser, Subcommand};


pub static CONFIG: LazyLock<config::Config> = LazyLock::new(|| {
    config::load_config()
});

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(
        about="Summarize content from various sources"
    )]
    Summarize {
        #[command(subcommand)]
        target: SummarizeTarget,
    },
}

#[derive(Subcommand)]
enum SummarizeTarget {
    #[command(
        about="Summarize content from youtube, provided a youtube or invidious url"
    )]
    Yt {
        url: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Summarize { target } => {
            match target {
                SummarizeTarget::Yt { url } => {
                    plugins::yt::run_summarize_yt(&CONFIG, &url);
                }
            }
        }
    }
}
