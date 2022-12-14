use crate::ArcStore;
use axum::{Router, routing::get};
use super::handlers::school_obtain;

pub fn app(store: ArcStore) -> Router {

    Router::new()
        .route("/obtain/:school", get(school_obtain))
        .with_state(store)
}
