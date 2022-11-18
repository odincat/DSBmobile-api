use rocket::{Catcher, Route, routes, catchers};

use super::{root::not_found, plan::get_plan};

pub fn server_routes() -> Vec<Route> {
    routes![get_plan]
}

pub fn catchers() -> Vec<Catcher> {
    return catchers![not_found];
}