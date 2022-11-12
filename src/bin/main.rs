use std::collections::HashMap;
use dsb_rs::{data::{requests::{TokenRequest, PlanRequest}, routines::fetch_and_parse}, Store};
use log::{info, error};
use serde_json::Value;
use simplelog::{CombinedLogger, TermLogger, WriteLogger, LevelFilter};

#[tokio::main]
async fn main() {
    TermLogger::init(LevelFilter::Info, simplelog::Config::default(), simplelog::TerminalMode::Mixed, simplelog::ColorChoice::Auto).unwrap();
    let config = dsb_rs::config::Config::load().await;

    // if &config.log_file.is_empty() == &false {
    //     let _ = WriteLogger::init(LevelFilter::Info, simplelog::Config::default(), std::fs::File::create(config.log_file).unwrap());
    // }

    let mut store = Store {
        plans: HashMap::new(),
    };
     
    store = fetch_and_parse(&config, store).await;
    println!("{:?}", store.plans[""]);
}
