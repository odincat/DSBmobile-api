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
        let date = chrono::offset::Utc::now().to_string(); 
        
        RequestObject {
            UserId: user_id.to_owned(),
            UserPw: user_password.to_owned(),
            AppVersion: "69.4.20".to_owned(),
            Language: "de".to_owned(),
            OsVersion: "69.4.20".to_owned(),
            AppId: "fc671af2-edc8-4536-9705-aac6db076817".to_owned(),
            Device: "Spaceshuttle".to_owned(),
            BundleId: "de.odincat.dsb-rs".to_owned(),
            Date: date.clone(),
            LastUpdate: date.clone()
        }
    }

    pub fn stringify(request_object: RequestObject) -> String {
        // TODO: Handle error
        let j = serde_json::to_string(&request_object).unwrap();
        return j;
    }
}