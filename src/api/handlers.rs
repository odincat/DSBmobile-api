use std::{convert::Infallible, collections::HashMap};
use warp::reply::json;
use crate::AppStore;

// We dont use our dummy type in the parameter here, because it is readable enough
pub async fn serve_school(school: String, query: HashMap<String, String>, store: AppStore) -> Result<impl warp::Reply, Infallible> {
    let store = store.lock().await;
    let store = store.clone();

    // if !store.plans.contains_key(&school) {
    //     return Ok(warp::reply::with_status("School unknown", StatusCode::NOT_FOUND));
    // }
    println!("{:?}", query);

    let mut plan = store.plans.get(&school).unwrap().clone();

    if query.get("select").is_some() {
        let class_query = query.get("select").unwrap().to_lowercase();
        let class_query = class_query.split(",");
        
        plan.current.content.retain(|item| {
            let class_string = item.get("klasse(n)").unwrap().to_lowercase();
            
            let mut ret = false;

            for class in class_query.clone() {
                let contains = class_string.contains(class);

                if contains {
                    ret = true;
                    break; 
                }

                ret = false; 
            }

            ret
        });

        plan.upcoming.content.retain(|item| {
            let class_string = item.get("klasse(n)").unwrap();
            
            let mut ret = false;

            for class in class_query.clone() {
                let contains = class_string.contains(class);

                if contains {
                    ret = true;
                    break; 
                }

                ret = false; 
            }

            ret
        });
        // plan.upcoming.content.retain(|item| item.get("klasse(n)").unwrap().contains(&class));
        // println!("{}: {:?}", &class, plan.current.content);
        // plan.current.content = plan.current.content
        //     .clone()
        //     .into_iter()
        //     .filter(|item| item.get("klasse(n)").unwrap().contains(&class))
        //     .collect::<Vec<HashMap<String, String>>>();
    }

    Ok(json(&plan))
}
