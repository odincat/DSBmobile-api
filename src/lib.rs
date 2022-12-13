use std::{collections::{HashMap, BTreeMap}, sync::Arc};

use scraper::ElementRef;
use serde::Serialize;
use tokio::sync::Mutex;

pub mod data;
pub mod api;
pub mod config;

pub mod protobuf;

pub fn get_text(element: &ElementRef) -> String {
    element.text().collect::<Vec<_>>()[0].to_string()
}

#[macro_export]
macro_rules! derive_alias {
    ($name:ident => #[derive($($derive:ident),*)]) => {
        macro_rules! $name {
            ($i:item) => {
                #[derive($($derive),*)]
                $i
            }
        }
    }
}

#[macro_export]
macro_rules! some_or_bail {
    ($option:expr, $fallback:expr) => {
        match $option{
            Some(value) => value,
            None => return $fallback 
        }
    }
}

/// Utility type representing the current (0) and the upcoming (1) plan
/// (current, upcoming)
pub type ValuePair<T> = (T, T);

#[derive(Clone, Debug)]
pub struct Store {
    pub schools: HashMap<String, SchoolResource>
}

impl Store {
    pub fn default() -> Store {
        Store {
            schools: HashMap::new()
        }
    }
}

pub type ArcStore = Arc<Mutex<Store>>;

#[derive(Clone, Debug, Serialize)]
pub struct SchoolResource {
    pub plan_url: String,
    pub last_updated: String,
    pub current: Plan,
    pub upcoming: Plan
}

pub type Substitutions = Vec<BTreeMap<String, String>>;

#[derive(Clone, Debug, Serialize)]
pub struct Plan {
    pub date: String,
    pub weekday: String,
    pub week_type: String,
    pub news: Vec<String>,
    pub content: Substitutions,
    pub affected_classes: Vec<String>
}

impl Plan {
    pub fn default() -> Plan {
        Plan {
            date: "".to_string(),
            weekday: "".to_string(),
            week_type: "".to_string(),
            news: vec![],
            content: vec![],
            affected_classes: vec![]
        }
    }
}

