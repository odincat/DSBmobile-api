use anyhow::{Result, Context, bail};
use log::info;
use reqwest::get;
use crate::{config::Config, Store, data::requests::{TokenRequest, PlanRequest}, Plan, data::parse::UntisParser};

pub async fn fetch_and_parse (config: &Config) -> Result<Store> {
    let mut store = Store::default();

    for provider in config.school_providers.iter() {
        let token = TokenRequest {
            username: provider.username.clone(),
            password: provider.password.clone()
        }.execute().await?;
        let plans = PlanRequest { token }.execute().await?;

        let children = plans[0]["Childs"]
            .as_array()
            .with_context(|| "Unable to access 'Childs' array in PlanRequest")?;

        let mut available_titles: Vec<String> = vec![];

        for plan in children.iter() {
            let plan_title = plan["Title"]
                .as_str()
                .with_context(|| "Unable to read field 'Title' in PlanRequest")?;

            available_titles.push(plan_title.to_string());

            if plan_title == provider.plan_title {
                let url = plan["Detail"].as_str()
                    .with_context(|| "Unable to read field 'Detail' in PlanRequest")?;
                let last_updated = plan["Date"].as_str()
                    .with_context(|| "Unable to read field 'Date' in PlanRequest")?;

                if url.is_empty() || last_updated.is_empty() {
                    bail!("Fields 'Detail' or 'Date' in PlanRequest are empty.")
                }

                let static_plan = get(url).await?
                    .text().await
                    .with_context(|| format!("Failed request static file from: {}", &url))?;

                // TODO: check for parser in config and use it accordingly to support multiple
                // types of plans.
                let parser = UntisParser { document: static_plan }.execute().await;

                let plan_object = Plan {
                    last_updated: last_updated.to_string(),
                    url: url.to_string(),
                    current: parser.current,
                    upcoming: parser.upcoming 
                };

                store.plans.insert(provider.url_identifier.clone(), plan_object);
            }
        }

        info!("Available titles for {}: {:?}", provider.url_identifier, available_titles);

        // if avaible_titles.contains(&provider.plan_title) == false {
        //     panic!("Plan title not found");
        // }
        if !available_titles.contains(&provider.plan_title) {
            println!("[WARN] Plan title '{}' not found", provider.plan_title)
        }
    }

    Ok(store)
}
