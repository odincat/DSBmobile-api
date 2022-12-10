use std::{collections::{HashMap, BTreeMap}, sync::Arc};

use scraper::ElementRef;
use serde::Serialize;
use tokio::sync::Mutex;

pub mod data;
pub mod api;
pub mod config;

pub fn err_panic(err: &str) {
    log::error!("PANIC: {}", err);
    panic!("{}", err);
}

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

pub type SubstitutionPlanContent = Vec<BTreeMap<String, String>>;

#[derive(Clone, Debug, Serialize)]
pub struct PlanContent {
    pub date: String,
    pub weekday: String,
    pub week_type: String,
    pub news: Vec<String>,
    pub content: SubstitutionPlanContent,
    pub affected_classes: Vec<String>
}

impl PlanContent {
    pub fn default() -> PlanContent {
        PlanContent {
            date: "".to_string(),
            weekday: "".to_string(),
            week_type: "".to_string(),
            news: vec![],
            content: vec![],
            affected_classes: vec![]
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Plan {
    pub url: String,
    pub last_updated: String,
    pub current: PlanContent,
    pub upcoming: PlanContent
}

#[derive(Clone, Debug)]
pub struct Store {
    pub plans: HashMap<String, Plan>
}

impl Store {
    pub fn default() -> Store {
        Store {
            plans: HashMap::new()
        }
    }
}

pub type AppStore = Arc<Mutex<Store>>;
