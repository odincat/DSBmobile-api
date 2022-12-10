use std::fs::read_to_string;
use anyhow::{Context, Result};
use serde::Deserialize;
use toml::from_str;

use crate::derive_alias;

derive_alias! {
    config_part => #[derive(Debug, Deserialize, Clone)]
}

config_part! {
    pub struct SchoolProvider {
        pub username: String,
        pub password: String, 
        pub plan_title: String,
        pub url_identifier: String 
    }
}

config_part! {
    pub struct Server {
        pub port: u16,
        pub host: String,
        pub refetch_interval: u64
    }
}

config_part! {
    pub struct Keys {
        pub enabled: bool,
        pub allowed: Vec<String>
    }
}

config_part! {
    pub struct Config {
        pub log_file: String,
        pub log_level: String, //TODO: impl 
        pub school_providers: Vec<SchoolProvider>,
        pub server: Server,
        pub keys: Keys
    }
}

impl Config {
    pub fn load() -> Result<Config> {
        let path = std::env::var("CONFIG_FILE").unwrap_or_else(|_| {
            println!("No config file overide was specified (environment variable: 'CONFIG_FILE'). Using default 'config.toml'.");
            "config.toml".to_string()
        });
        let file = read_to_string(&path).with_context(|| format!("Unable to read config file '{}'", &path))?;
        let config: Config = from_str(&file).with_context(|| format!("An error occured while parsing config ('{}'). For correct format please refer to the example.config.toml", &path))?;

        Ok(config)
    }
}
