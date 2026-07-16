mod yt;
mod ai_calls;
mod config;
use std::sync::LazyLock;

pub static CONFIG: LazyLock<config::Config> = LazyLock::new(|| {
    config::load_config()
});

fn main() {
    let result = yt::get_subtitles("https://www.youtube.com/watch?v=QhEWeDT_9gI").unwrap();

    match ai_calls::use_pattern("yt-summary", &result, &CONFIG) {
        Ok(x) => println!("{x}"),
        Err(x) => println!("ERROR: {x}"),
    }
}
