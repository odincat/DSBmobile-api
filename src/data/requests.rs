use reqwest::Client;
use serde_json::Value;
use crate::err_panic;

pub struct TokenRequest {
    pub username: String,
    pub password: String 
}

impl TokenRequest {
    fn build_url(&self) -> String {
        if(self.username.len() == 0) || (self.password.len() == 0) {
            err_panic("Username or password is empty");
        }

        let base_url = "https://mobileapi.dsbcontrol.de/authid";
        let additional_params = "&bundleid=de.heinekingmedia.dsbmobile&appversion=35&osversion=22&pushid";

        return format!("{}/?user={}&password={}&{}", base_url, self.username, self.password, additional_params);
    }

    pub async fn execute(&self) -> String {
        let url = self.build_url();
        let client = Client::new();

        let response = client.get(url).send().await.unwrap().text().await.unwrap();
        let parsed_response: String = serde_json::from_str(response.as_str()).unwrap();

        return parsed_response;
    }
}

pub struct PlanRequest {
    pub token: String 
}

impl PlanRequest {
    fn build_url(&self) -> String {
        if self.token.len() == 0 {
            err_panic("Token must be supplied");
        }

        let base_url = "https://mobileapi.dsbcontrol.de/dsbtimetables";

        return format!("{}/?authid={}", base_url, self.token);
    }

    pub async fn execute(&self) -> Value {
        let url = self.build_url();
        let client = Client::new();

        let response = client.get(url).send().await.unwrap().text().await.unwrap();
        let parsed_response: Value = serde_json::from_str(response.as_str()).unwrap();

        return parsed_response;
    }
}