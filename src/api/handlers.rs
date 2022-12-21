use axum::{extract::{Path, State, Query}, http::StatusCode, response::IntoResponse, Json};
use axum_extra::protobuf::ProtoBuf;
use serde::Deserialize;
use crate::{AppStore, SubstitutionPlanContent, protobuf::untis};

fn select_classes(content: &mut Substitutions, classes: &Vec<&str>) -> Substitutions {
    content.retain(|item| {
        let item_class = item.get("klasse(n)").unwrap().to_lowercase().to_string();
        classes.iter().any(|class| item_class.contains(class.to_lowercase().as_str()))
    });

    content.clone()
}

fn remove_classes(content: &mut Substitutions, classes: &Vec<&str>) -> Substitutions {
    content.retain(|item| {
        let item_class = item.get("klasse(n)").unwrap().to_lowercase().to_string();
        !classes.iter().any(|class| item_class.contains(class))
    });

    content.clone()
}

#[derive(Debug, Deserialize)]
pub struct SchoolObtainParams {
    pub select: Option<String>,
    pub remove: Option<String>,
    pub proto: Option<String>
}

pub async fn school_obtain(Path(school_identifier): Path<String>, Query(params): Query<SchoolObtainParams>, State(store): State<ArcStore>) -> impl IntoResponse {
    let store = store.lock().await;
    let store = store.clone();
    println!("GET, {:?}", params);

    if !store.schools.contains_key(&school_identifier) {
        return (StatusCode::NOT_FOUND, "School unknown").into_response();
    }

    let mut plan = store.schools.get(&school_identifier).unwrap().clone();

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

    if params.proto.is_some() {
        let current_content: Vec<untis::Substitution> = plan.current.content.into_iter().map(|item| {
            let item = item.clone();
            untis::Substitution {
                klasse: item.get("klasse(n)").unwrap().to_string(),
                stunde: item.get("stunde").unwrap().to_string(),
                fach: item.get("fach").unwrap().to_string(),
                fach_alt: item.get("(fach)").unwrap().to_string(),
                raum: item.get("raum").unwrap().to_string(),
                raum_alt: item.get("(raum)").unwrap().to_string(),
                vertr_von: item.get("vertr. von").unwrap().to_string(),
                art: item.get("art").unwrap().to_string(),
                text: item.get("text").unwrap().to_string()
            }
        }).collect();

        let current_plan = untis::Plan {
            date: plan.current.date,
            weekday: plan.current.weekday,
            week_type: plan.current.week_type,
            news: plan.current.news,
            affected_classes: plan.current.affected_classes,
            content: current_content 
        };
        let current_plan = Option::from(current_plan);

        let upcoming_content: Vec<untis::Substitution> = plan.upcoming.content.into_iter().map(|item| {
            let item = item.clone();
            untis::Substitution {
                klasse: item.get("klasse(n)").unwrap().to_string(),
                stunde: item.get("stunde").unwrap().to_string(),
                fach: item.get("fach").unwrap().to_string(),
                fach_alt: item.get("(fach)").unwrap().to_string(),
                raum: item.get("raum").unwrap().to_string(),
                raum_alt: item.get("(raum)").unwrap().to_string(),
                vertr_von: item.get("vertr. von").unwrap().to_string(),
                art: item.get("art").unwrap().to_string(),
                text: item.get("text").unwrap().to_string()
            }
        }).collect();

        let upcoming_plan = untis::Plan {
            date: plan.upcoming.date,
            weekday: plan.upcoming.weekday,
            week_type: plan.upcoming.week_type,
            news: plan.upcoming.news,
            affected_classes: plan.upcoming.affected_classes,
            content: upcoming_content
        };
        let upcoming_plan = Option::from(upcoming_plan);

        let proto_overview = untis::Overview {
            url: plan.url,
            last_updated: plan.last_updated,
            current: current_plan,
            upcoming: upcoming_plan 
        };

        return (StatusCode::OK, ProtoBuf(proto_overview)).into_response()
    }

    (StatusCode::OK, Json(&plan)).into_response()
} 
