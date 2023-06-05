use crate::models::post::Post;
use crate::service;
use rocket::{self, post, serde::json::Json};

#[post("/post", format = "json", data = "<post>")]
pub async fn add_post(
    post: Json<Post>,
    post_service: &rocket::State<service::post::PostService>,
    user_service: &rocket::State<service::user::UserService>,
) -> Result<Json<Post>, Json<&'static str>> {
    let user = user_service.get_user(post.0.user_id);
    match user {
        Some(_) => return Ok(Json(post_service.add_post(post.0))),
        None => return Err(Json("User does not exist")),
    }
}

#[rocket::get("/posts")]
pub async fn get_posts(service: &rocket::State<service::post::PostService>) -> Json<Vec<Post>> {
    Json(service.get_posts())
}

#[rocket::get("/post/<id>")]
pub async fn get_post(
    id: i32,
    service: &rocket::State<service::post::PostService>,
) -> Option<Json<Post>> {
    service.get_post(id).map(|p| Json(p))
}
