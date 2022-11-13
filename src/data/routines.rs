use log::info;

use crate::{config::Config, Store, data::requests::{TokenRequest, PlanRequest}, Content, Plan, data::parse::GenericPlanParser};

pub async fn fetch_and_parse (config: &Config, inital_store: Store) -> Store {
    let mut store = inital_store;

    for provider in config.school_providers.iter() {
        let token = TokenRequest { username: provider.username.clone(), password: provider.password.clone() }.execute().await;
        let plans = PlanRequest { token }.execute().await;

        let children = plans[0]["Childs"].as_array().unwrap();
        let mut avaible_titles: Vec<String> = vec![];

        for plan in children.iter() {
            let plan_title = plan["Title"].as_str().unwrap();
            avaible_titles.push(plan_title.to_string());

            if plan_title == provider.plan_title {
                // store.plans.insert(provider.url_identifier.clone(), );

                let url = plan["Detail"].as_str().unwrap();
                let last_updated = plan["Date"].as_str().unwrap();

                let mut plan_object = Plan {
                    last_updated: last_updated.to_string(),
                    url: url.to_string(),
                    current: Content::default(),
                    upcoming: Content::default() 
                };

                let parser = GenericPlanParser { url: url.to_owned() }.execute().await;

                plan_object.current = parser.current;
                plan_object.upcoming = parser.upcoming;

                store.plans.insert(provider.url_identifier.clone(), plan_object);
            }
        }

        info!("Available titles for {}: {:?}", provider.url_identifier, avaible_titles);

        // if avaible_titles.contains(&provider.plan_title) == false {
        //     panic!("Plan title not found");
        // }
    }

    return store;
}