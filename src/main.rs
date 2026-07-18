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
    /// Get Weather Data
    Weather,
    /// Get your public ip address
    Ip {
        #[arg(short = '6', help = "Get IpV6 information")]
        ip_v6: bool,
    }
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
            }
        }
    }
}
