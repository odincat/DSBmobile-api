use rocket::{catch};

#[catch(404)]
pub fn not_found() -> String {
    return "STATUS: 404; The requested resource was not found :/".to_string();
}