use rocket::{get, State};

use crate::store::Store;

#[get("/plan/<school_ident>/<plan_version>/<class>")]
pub fn get_plan(school_ident: &str, plan_version: &str, class: &str) {
    
    println!("{:?}", store);
    
}