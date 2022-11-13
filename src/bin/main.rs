use std::collections::HashMap;
use dsb_rs::{data::routines::fetch_and_parse, Store};
use simplelog::{TermLogger, LevelFilter};
use tokio::{task, time};

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
     
    let refetch = task::spawn(async move {
        let mut interval = time::interval(time::Duration::from_secs(config.server.refetch_interval));

        loop {
            interval.tick().await;
            println!("Fetching...");
            store = fetch_and_parse(&config, store).await;
        }
    });

    refetch.await.unwrap();
}
