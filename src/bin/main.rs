#[macro_use] extern crate rocket;

use std::{collections::HashMap, sync::{Arc, Mutex}};
use arc_swap::{ArcSwap};
use dsb_rs::{data::routines::fetch_and_parse, Store, api::routes::{server_routes, catchers}, config::Config as AppConfig};
use rocket::Config;
use simplelog::{TermLogger, LevelFilter};
use tokio::{task, time};
use lazy_static::{lazy_static, lazy::Lazy};
    
lazy_static! {
    #[derive(Debug)]
    pub static ref STORE: ArcSwap<Store> = {
        ArcSwap::from(Arc::new(Store {
            plans: HashMap::new()
        }))
    };

    pub static ref CONFIG: AppConfig = {
        let config = AppConfig::load();
        config
    };
}

#[tokio::main]
async fn main() {
    TermLogger::init(LevelFilter::Info, simplelog::Config::default(), simplelog::TerminalMode::Mixed, simplelog::ColorChoice::Auto).unwrap();

    // if &config.log_file.is_empty() == &false {
    //     let _ = WriteLogger::init(LevelFilter::Info, simplelog::Config::default(), std::fs::File::create(config.log_file).unwrap());
    // }

    // let mut store = Arc::new(Mutex::new(Store {
    //     plans: HashMap::new(),
    // }));


    let _refetch = task::spawn(async move {
        let mut interval = time::interval(time::Duration::from_secs(CONFIG.server.refetch_interval));

        loop {
            interval.tick().await;
            println!("Refetching...");

            let new_store = fetch_and_parse(&CONFIG).await;
            STORE.swap(Arc::new(new_store));
        }
    });


    let server_config = Config {
        port: CONFIG.server.port,
        cli_colors: false,
        address: CONFIG.server.host.parse().unwrap(),
        ..Config::default()
    };

    let _rocket = rocket::custom(&server_config)
        .register("/", catchers())
        .mount("/", server_routes())
        .manage(STORE.load())
        .launch()
        .await
        .unwrap();

}
