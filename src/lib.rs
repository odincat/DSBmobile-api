use std::collections::HashMap;

use scraper::ElementRef;
use serde::Serialize;

pub mod api;
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

#[derive(Debug, Serialize)]
pub struct Content {
    pub news: Vec<String>,
    pub date: String,
    pub weekday: String,
    pub content: Vec<HashMap<String, String>>,
    pub week_type: String
}

impl Content {
    pub fn default() -> Content {
        Content {
            news: vec![],
            date: "".to_string(),
            weekday: "".to_string(),
            content: vec![],
            week_type: "".to_string()
        }
    }
}

#[derive(Debug, Serialize)]
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