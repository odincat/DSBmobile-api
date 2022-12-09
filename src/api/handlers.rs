use axum::{extract::{Path, State, Query}, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use crate::{AppStore, SubstitutionPlanContent};

fn select_classes(content: &mut SubstitutionPlanContent, classes: &Vec<&str>) -> SubstitutionPlanContent {
    content.retain(|item| {
        let item_class = item.get("klasse(n)").unwrap().to_lowercase().to_string();
        classes.iter().any(|class| item_class.contains(class.to_lowercase().as_str()))
    });

    content.clone()
}

fn remove_classes(content: &mut SubstitutionPlanContent, classes: &Vec<&str>) -> SubstitutionPlanContent {
    content.retain(|item| {
        let item_class = item.get("klasse(n)").unwrap().to_lowercase().to_string();
        !classes.iter().any(|class| item_class.contains(class))
    });

    content.clone()
}

#[derive(Debug, Deserialize)]
pub struct SchoolObtainParams {
    pub select: Option<String>,
    pub remove: Option<String>
}

pub async fn school_obtain(Path(school_identifier): Path<String>, Query(params): Query<SchoolObtainParams>, State(store): State<AppStore>) -> impl IntoResponse {
    let store = store.lock().await;
    let store = store.clone();
    println!("hi, {:?}", params);

    if !store.plans.contains_key(&school_identifier) {
        return (StatusCode::NOT_FOUND, "School unknown").into_response();
    }

    let mut plan = store.plans.get(&school_identifier).unwrap().clone();

    if params.remove.is_some() {
        let classes = params.remove.as_ref().unwrap().split(",").collect::<Vec<&str>>();

        plan.current.content = remove_classes(&mut plan.current.content, &classes);
        plan.upcoming.content = remove_classes(&mut plan.upcoming.content, &classes);
    }

    if params.select.is_some() {
        let classes = params.select.as_ref().unwrap().split(",").collect::<Vec<&str>>();

        plan.current.content = select_classes(&mut plan.current.content, &classes);
        plan.upcoming.content = select_classes(&mut plan.upcoming.content, &classes);
    }

    (StatusCode::OK, Json(&plan)).into_response()
} 
