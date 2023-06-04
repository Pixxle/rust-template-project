use rocket::{self};
use crate::service;

#[rocket::post("/users")]
pub async fn add_user(service: &rocket::State<service::user::UserService>) -> String {
    service.add_user(&"test".to_string());
    "User added".to_string()
}


#[rocket::get("/users")]
pub async fn get_users(service: &rocket::State<service::user::UserService>) -> String {
    format!("Posts: {:?}", service.get_users())
}