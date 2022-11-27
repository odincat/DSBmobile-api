use std::{convert::Infallible, collections::HashMap};
use warp::{reply::json, Reply, query};
use crate::{AppStore};

// We dont use our dummy type in the parameter here, because it is readable enough
pub async fn serve_school(school: String, query: HashMap<String, String>, store: AppStore) -> Result<impl warp::Reply, Infallible> {
    let store = store.lock().await;
    let store = store.clone();

    // if !store.plans.contains_key(&school) {
    //     return Ok(warp::reply::with_status("School unknown", StatusCode::NOT_FOUND));
    // }
    println!("{:?}", query);

    let mut plan = store.plans.get(&school).unwrap().clone();

    if query.get("class").is_some() {
        let class = query.get("class").unwrap().to_lowercase();
    
        plan.current.content = plan.current.content
            .clone()
            .into_iter()
            .filter(|item| item.get("klasse(n)").unwrap().contains(&class))
            .collect::<Vec<HashMap<String, String>>>();
    }

    Ok(json(&plan))
}