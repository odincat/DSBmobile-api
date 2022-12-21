use anyhow::{bail, Context, Result};
use reqwest::get;
use serde_json::Value;

pub struct TokenRequest {
    pub username: String,
    pub password: String 
}

impl TokenRequest {
    fn build_url(&self) -> Result<String> {
        if(self.username.len() == 0) || (self.password.len() == 0) {
            bail!("Username or password is empty, please check your config file")
        }

        const BASE_URL: &str = "https://mobileapi.dsbcontrol.de/authid";
        const ADDITIONAL_PARAMS: &str = "&bundleid=de.heinekingmedia.dsbmobile&appversion=35&osversion=22&pushid";

        Ok(format!("{}/?user={}&password={}&{}", BASE_URL, self.username, self.password, ADDITIONAL_PARAMS))
    }

    pub async fn execute(&self) -> Result<String> {
        let url = self.build_url()?;

        let response = get(&url).await
            .with_context(|| format!("Failed to fetch token from: {}", &url))? 
            .text().await?;

        let parsed_response: String = serde_json::from_str(response.as_str())?;

        Ok(parsed_response)
    }
}

pub struct PlanRequest {
    pub token: String 
}

impl PlanRequest {
    fn build_url(&self) -> Result<String> {
        if self.token.len() == 0 {
            bail!("Token must be supplied")
        }

        const BASE_URL: &str = "https://mobileapi.dsbcontrol.de/dsbtimetables";

        Ok(format!("{}/?authid={}", BASE_URL, self.token))
    }

    pub async fn execute(&self) -> Result<Value> {
        let url = self.build_url().unwrap();

        let response = get(&url).await
            .with_context(|| format!("Failed to fetch plans from: {}", &url))?
            .text().await?;

        let parsed_response: Value = serde_json::from_str(response.as_str()).unwrap();

        Ok(parsed_response)
    }
}

#[cfg(test)]
mod tests {
    use super::{TokenRequest, PlanRequest};

    #[test]
    fn token_input_check () {
        let token_request = TokenRequest {
            username: "".to_string(),
            password: "".to_string()
        };

        assert!(token_request.build_url().is_err());

        let token_request = TokenRequest {
            username: "1111".to_string(),
            password: "".to_string()
        };

        assert!(token_request.build_url().is_err())
    }

    #[test]
    fn plan_input_check () {
        let plan_request = PlanRequest {
            token: "".to_string()
        };

        assert!(plan_request.build_url().is_err())
    }
}
