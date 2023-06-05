use crate::models::user::User;
use crate::service;
use rocket::{self, get, post, serde::json::Json};

#[post("/user", format = "json", data = "<user>")]
pub async fn add_user<'a>(
    user: Json<User>,
    service: &rocket::State<service::user::UserService>,
) -> Json<User> {
    let resp = service.add_user(user.0);
    Json(resp)
}

#[get("/users")]
pub async fn get_users(service: &rocket::State<service::user::UserService>) -> Json<Vec<User>> {
    Json(service.get_users())
}

#[get("/user/<id>")]
pub async fn get_user(
    id: i32,
    service: &rocket::State<service::user::UserService>,
) -> Option<Json<User>> {
    service.get_user(id).map(|u| Json(u))
}
