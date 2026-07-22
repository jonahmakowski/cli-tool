use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use directories::ProjectDirs;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;

const TVBD_API_BASE: &str = "https://api4.thetvdb.com/v4";
const THREE_WEEKS: u64 = 3 * 7 * 24 * 60 * 60;

#[derive(Serialize)]
struct ApiKey<'a> {
    apikey: &'a str,
}

#[derive(Deserialize, Clone)]
struct BearerResponse {
    data: BearerData,
}

#[derive(Deserialize, Clone)]
struct BearerData {
    token: String,
}

#[derive(Serialize, Deserialize)]
struct CachedToken {
    token: String,
    created_at: u64,
}

impl CachedToken {
    fn is_valid(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();
        
        if now - self.created_at < THREE_WEEKS {
            return Ok(true);
        }
        Ok(false)
    }
}

pub fn get_bearer(api_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let dirs = ProjectDirs::from("com", "jonahmakowski", "cli-tool").unwrap();
    let cache_dir = dirs.cache_dir();
    let cache_file = cache_dir.join("tvdb_cache.json");

    if cache_file.is_file() {
        let file_data: CachedToken = serde_json::from_str(&std::fs::read_to_string(&cache_file)?)?;
        if file_data.is_valid()? {
            return Ok(file_data.token);
        }
    }

    println!("Cache file dosen't exist or key is outdated, running internet mode");

    let client = Client::new();

    let data = client
        .post(format!("{}{}", TVBD_API_BASE, "/login"))
        .json(&ApiKey {apikey: api_key})
        .send()?
        .text()?;

    let token  = serde_json::from_str::<BearerResponse>(&data)?.data.token;

    let new_cache = CachedToken {
        token: token.clone(),
        created_at: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
    };

    fs::create_dir_all(&cache_dir)?;
    fs::write(&cache_file, serde_json::to_vec_pretty(&new_cache).unwrap())?;

    return Ok(token);
}
