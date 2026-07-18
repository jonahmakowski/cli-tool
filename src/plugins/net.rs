use reqwest::blocking::Client;
use reqwest::header;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use serde::Deserialize;

pub enum IpType {
    V4,
    V6,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IpInfo {
    latitude: f64,
    longitude: f64,
    country_name: String,
    city_name: String,
    region_name: String,
}

#[derive(Debug, Deserialize)]
struct WeatherData {
    current: CurrentWeather,
    daily: DailyWeather,
}

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    temperature_2m: f64,
    relative_humidity_2m: i64,
    wind_speed_10m: f64,
    wind_direction_10m: i64,
    precipitation: f64,
    apparent_temperature: f64,
}

#[derive(Debug, Deserialize)]
struct DailyWeather {
    temperature_2m_max: Vec<f64>,
    temperature_2m_min: Vec<f64>,
    temperature_2m_mean: Vec<f64>,
}

pub fn get_public_ip(ip_type: &IpType) -> Result<String, Box<dyn std::error::Error>> {
    let mut client_builder = Client::builder();

    match ip_type {
        IpType::V4 => client_builder = client_builder.local_address(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
        IpType::V6 => client_builder = client_builder.local_address(IpAddr::V6(Ipv6Addr::UNSPECIFIED)),
    }
        
    let client = client_builder.build()?;

    let ip = client.get("https://ifconfig.me")
        .header(header::USER_AGENT, "curl/8.7.1")
        .send()?
        .text()?;

    Ok(ip)
}

fn get_ip_info() -> Result<IpInfo, Box<dyn std::error::Error >> {
    let public_ip = get_public_ip(&IpType::V4)?;

    let client = Client::new();

    let data = client.get(format!("https://free.freeipapi.com/api/json/{}", public_ip))
        .send()?
        .text()?;

    let info: IpInfo = serde_json::from_str(&data)?;

    Ok(info)
}

fn query_weather(ip_info: &IpInfo) -> Result<WeatherData, Box<dyn std::error::Error >> {
    let client = Client::new();

    let result = client
        .get(format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&daily=temperature_2m_max,temperature_2m_min,temperature_2m_mean&current=temperature_2m,relative_humidity_2m,wind_speed_10m,wind_direction_10m,precipitation,apparent_temperature&timezone=auto", ip_info.latitude, ip_info.longitude))
        .send()?
        .text()?;

    let info: WeatherData = serde_json::from_str(&result)?;

    Ok(info)
}

pub fn get_weather_data() {
    match get_ip_info() {
        Ok(ip_info) => {
            match query_weather(&ip_info) {
                Ok(data) => {
                    println!("Weather in {}, {}, {}", ip_info.city_name, ip_info.region_name, ip_info.country_name);
                    println!("Current temperature: {}C", data.current.temperature_2m);
                    println!("It feels like: {}C", data.current.apparent_temperature);
                    println!("Current humidity: {}%", data.current.relative_humidity_2m);
                    println!("Current rainfall: {}mm", data.current.precipitation);
                    println!("Current windspeed: {} kph", data.current.wind_speed_10m);
                    println!("Current wind direction: {} degrees", data.current.wind_direction_10m);
                    println!("Max temperature today: {}C", data.daily.temperature_2m_max[0]);
                    println!("Min temperature today: {}C", data.daily.temperature_2m_min[0]);
                    println!("Mean temperature today: {}C", data.daily.temperature_2m_mean[0]);
                }
                Err(err) => eprintln!("Error: {}", err),
            }
        }
        Err(err) => eprintln!("Error: {}", err)
    }
}
