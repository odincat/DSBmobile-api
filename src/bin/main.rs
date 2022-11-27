use dsb_rs::{api::routes::get_school, config::Config, data::routines::fetch_and_parse, Store};
use lazy_static::lazy_static;
use simplelog::{LevelFilter, TermLogger};
use std::{collections::HashMap, sync::Arc};
use tokio::{sync::Mutex, task, time};

lazy_static! {
    pub static ref CONFIG: Config = {
        let config = Config::load();
        config
    };
}

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
    let _refetch = task::spawn(async move {
        let mut interval =
            time::interval(time::Duration::from_secs(config.server.refetch_interval));

        loop {
            interval.tick().await;
            println!("Refetching...");

            let new_store = fetch_and_parse(&CONFIG).await;
            let mut store = thread_store.lock().await;
            *store = new_store;
        }
    });

    // let parsed_host: Vec<i32> = config.server.host.split(".").map(|s| s.parse().expect("unable to parse host (expected number, found string")).collect();

    warp::serve(get_school(store.clone()))
        .run(([127, 0, 0, 1], config.server.port))
        .await;
}
