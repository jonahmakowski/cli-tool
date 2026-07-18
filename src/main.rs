mod ai_calls;
mod config;
mod plugins;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(
        short = 'p',
        long, 
        global=true,
        help="Use the private AI configuration in relevant workflows"
    )]
    private:bool,
}

#[derive(Subcommand)]
enum Command {
    /// Summarize content from various sources
    Summarize {
        #[command(subcommand)]
        target: SummarizeTarget,
    },
    // Get data from the internet
    Net {
        #[command(subcommand)]
        target: NetTarget,
    }
}

#[derive(Subcommand)]
enum SummarizeTarget {
    #[command(
        about="Summarize content from youtube, provided a youtube or invidious url"
    )]
    Yt {
        /// Url of the youtube video
        url: String,
    },
}

#[derive(Subcommand)]
enum NetTarget {
    #[command(
        about="Get weather data"
    )]
    Weather,
}

fn main() {
    let cli = Cli::parse();
    let config = config::load_config(cli.private);

    match cli.command {
        Command::Summarize { target } => {
            match target {
                SummarizeTarget::Yt { url } => {
                    plugins::yt::run_summarize_yt(&config, &url);
                }
            }
        }
        Command::Net { target } => {
            match target {
                NetTarget::Weather => {
                    plugins::net::get_weather_data();
                }
            }
        }
    }
}
