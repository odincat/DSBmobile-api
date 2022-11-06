use serde::{Deserialize, Serialize};

// Request
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)] pub struct RequestObject {
    UserId: String,
    UserPw: String,
    AppVersion: String,
    Language: String,
    OsVersion: String,
    AppId: String,
    Device: String,
    BundleId: String,
    Date: String,
    LastUpdate: String,
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
            AppVersion: "2.3.4".to_owned(),
            Language: "en-DE".to_owned(),
            OsVersion: "69.4.2".to_owned(),
            AppId: "fc671af2-edc8-4536-9705-aac6db076817".to_owned(),
            Device: "Spaceshuttle".to_owned(),
            BundleId: "de.odincat.dsb-rs".to_owned(),
            Date: "Sun Nov 06 2022 00:15:28 GMT+0100 (Central European Standard Time)".to_owned(),
            LastUpdate: "Sun Nov 06 2022 00:15:28 GMT+0100 (Central European Standard Time)".to_owned(), 
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
}