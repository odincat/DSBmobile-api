use axum::Server;
use dsb_rs::{api::routes::app, config::Config, data::routines::fetch_and_parse, Store};
use std::sync::Arc;
use tokio::{sync::Mutex, task, time};

#[tokio::main]
async fn main() {
    let config = Config::load().unwrap();

    let store = Store::default();
    let store = Mutex::new(store);
    let store = Arc::new(store);

    let thread_store = store.clone();
    let thread_config = config.clone();
    let _refetch = task::spawn(async move {
        let mut interval = time::interval(time::Duration::from_secs(config.server.refetch_interval));

        loop {
            interval.tick().await;

            let new_store = match fetch_and_parse(&thread_config).await {
                Ok(store) => store,
                Err(e) => {
                    println!("[FATAL] Unable to grab data: {}", e);
                    continue;
                },
            };

            let mut store = thread_store.lock().await;
            *store = new_store;
        }
    });

    // TODO: Dont't start server until data is available

    let host = format!("{}:{}", &config.server.host, &config.server.port.to_string());

    println!("Launching server on {}", &host);

    Server::bind(&host.parse().unwrap())
        .serve(app(store.clone()).into_make_service())
        .await
        .unwrap();
}
