use rocket::{self};
use crate::service;

#[rocket::post("/posts")]
pub async fn add_post(service: &rocket::State<service::post::PostService>) -> String {
    service.add_post(&"test".to_string());
    "Post added".to_string()
}


#[rocket::get("/posts")]
pub async fn get_posts(service: &rocket::State<service::post::PostService>) -> String {
    format!("Posts: {:?}", service.get_posts())
}

