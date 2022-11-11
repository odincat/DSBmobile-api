use std::collections::HashMap;

use serde_json::Value;

pub mod data;
pub mod config;

pub fn err_panic(err: &str) {
    log::error!("PANIC: {}", err);
    panic!("{}", err);
}

#[derive(Debug)]
pub struct Content {
    pub info: Vec<String>,
    pub content: Value,
}

#[derive(Debug)]
pub struct Plan {
    pub url: String,
    current: Content,
    upcoming: Content,
    last_updated: String
}

#[derive(Debug)]
pub struct Store {
    pub plans: HashMap<String, Plan>
}