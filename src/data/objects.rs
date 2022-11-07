use serde::{Deserialize, Serialize};

// Request
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)] pub struct RequestObject {
    UserId: String,
    UserPw: String,
    PushId: String,
    AppId: String,
    AppVersion: String,
    BundleId: String,
    Date: String,
    Device: String,
    Language: String,
    LastUpdate: String,
    OsVersion: String,
}

impl RequestObject {
    pub fn new(user_id: &str, user_password: &str) -> RequestObject {
        let date = chrono::offset::Utc::now(); 
        
        if (user_id.len() == 0) || (user_password.len() == 0) {
            panic!("User ID or Password is empty");
        }

        RequestObject {
            UserId: user_id.to_owned(),
            UserPw: user_password.to_owned(),
            AppId: "BC86F8E5-5D4A-4A19-A317-04D1E52FF9ED".to_owned(),
            AppVersion: "2.5.6".to_owned(),
            // https://itunes.apple.com/lookup?id=461741785
            BundleId: "de.digitales-schwarzes-brett.dsblight".to_owned(),
            Date: date.to_string(),
            Device: "iPhone".to_owned(),
            Language: "en-DE".to_owned(),
            LastUpdate: date.to_string(), 
            OsVersion: "13.2.2".to_owned(),
            PushId: "".to_owned(),
        }
    }

    pub fn stringify(request_object: RequestObject) -> String {
        // TODO: Handle possible error
        return serde_json::to_string(&request_object).unwrap();
    }
}

// Response
#[derive(Debug, Deserialize, Serialize)] pub struct ResponseObject {
    d: String
}

impl ResponseObject {
    pub fn parse(response: &str) -> ResponseObject {
        let response_object: ResponseObject = serde_json::from_str(response).unwrap();
        return response_object
    }

    pub fn retrieve_plan_url (responseObject: ResponseObject) -> String {
        return String::from("asdd_");
    }
}