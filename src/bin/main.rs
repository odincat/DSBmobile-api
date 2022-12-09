use axum::Server;
use dsb_rs::{api::routes::app, config::Config, data::routines::fetch_and_parse, Store};
use simplelog::{LevelFilter, TermLogger};
use std::{collections::HashMap, sync::Arc};
use tokio::{sync::Mutex, task, time};

#[tokio::main]
async fn main() {
    TermLogger::init(
        LevelFilter::Info,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    // if &config.log_file.is_empty() == &false {
    //     let _ = WriteLogger::init(LevelFilter::Info, simplelog::Config::default(), std::fs::File::create(config.log_file).unwrap());
    // }

    // let mut store = Arc::new(Mutex::new(Store {
    //     plans: HashMap::new(),
    // }));

    let config = Config::load();

    let store = Store {
        plans: HashMap::new(),
    };
    let store = Mutex::new(store);
    let store = Arc::new(store);

    let thread_store = store.clone();
    let thread_config = config.clone();
    let _refetch = task::spawn(async move {
        let mut interval =
            time::interval(time::Duration::from_secs(config.server.refetch_interval));

        loop {
            interval.tick().await;
            println!("Refetching...");

            let new_store = fetch_and_parse(&thread_config).await;
            let mut store = thread_store.lock().await;
            *store = new_store;
        }
    });

    let host = format!("{}:{}", &config.server.host, &config.server.port.to_string());
    Server::bind(&host.parse().unwrap())
        .serve(app(store.clone()).into_make_service())
        .await
        .unwrap();
}
