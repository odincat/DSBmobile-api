use std::collections::HashMap;

use scraper::ElementRef;
use serde_json::Value;

pub mod data;
pub mod config;

pub fn err_panic(err: &str) {
    log::error!("PANIC: {}", err);
    panic!("{}", err);
}

pub fn get_text(element: &ElementRef) -> String {
    let text = element.text().collect::<Vec<_>>()[0].to_string();
    text
}

#[derive(Debug)]
pub struct Content {
    pub news: Vec<String>,
    pub date: String,
    pub weekday: String,
    pub content: Value,
    pub week_type: String
}

#[derive(Debug)]
pub struct Plan {
    pub url: String,
    pub current: Content,
    pub upcoming: Content,
    pub last_updated: String
}

#[derive(Debug)]
pub struct Store {
    pub plans: HashMap<String, Plan>
}