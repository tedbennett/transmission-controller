use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub tv_path: String,
    pub movies_path: String,
    pub vpn_ip: String,
}

pub fn parse_config() -> Option<Config> {
    let contents = fs::read_to_string("config.json").ok()?;
    serde_json::from_str(&contents).ok()
}
