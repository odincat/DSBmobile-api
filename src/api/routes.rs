use std::{convert::Infallible, collections::HashMap};
use warp::{Filter, Reply, Rejection, path, get, any, query};
use crate::AppStore;
use super::handlers;

pub fn with_data_store(store: AppStore) -> impl Filter<Extract = (AppStore,), Error = Infallible> + Clone {
    any().map(move || store.clone())
}

// GET /obtain/{school}?class={class}
// This is unnecessary, but makes the route path more readable
type School = String;

pub fn get_school(store: AppStore) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!("obtain" / School)
        .and(get())
        .and(query::<HashMap<String, String>>())
        .and(with_data_store(store))
        .and_then(handlers::serve_school)
}
// END: GET /{school}?class={class}
