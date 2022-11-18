use std::fs::read_to_string;
use serde::Deserialize;
use toml::from_str;

#[derive(Deserialize, Debug)]
pub struct SchoolProvider {
    pub username: String,
    pub password: String, 
    pub plan_title: String,
    pub url_identifier: String 
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub port: u16,
    pub host: String,
    pub refetch_interval: u64
}

#[derive(Deserialize, Debug)]
pub struct Keys {
    pub enabled: bool,
    pub allowed: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub log_file: String,
    pub log_level: String, //TODO: impl 
    pub school_providers: Vec<SchoolProvider>,
    pub server: Server,
    pub keys: Keys
}

impl Config {
    pub fn load() -> Config {
        let config_file_path = std::env::var("CONFIG_FILE").unwrap_or("config.toml".to_string());
        let config_file = read_to_string(config_file_path).unwrap();
        let config: Config = from_str(&config_file).unwrap();

        return config;
    }
}