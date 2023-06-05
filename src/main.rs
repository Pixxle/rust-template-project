use rocket::{self};
use std::sync::Arc;

mod api;
mod config;
mod repository;
mod service;

#[rocket::launch]
fn rocket() -> _ {
    let config = config::parse();

    let db: Arc<dyn repository::Database> = match config.environment {
        config::Environment::Development => Arc::new(repository::database::in_mem_database::new()),
        config::Environment::Production => unimplemented!(),
    };

    let user_service = service::user::new(db.clone());
    let post_service = service::post::new(db.clone());

    rocket::build()
        .manage(user_service)
        .manage(post_service)
        .mount("/", rocket::routes![api::post::get_posts])
        .mount("/", rocket::routes![api::post::add_post])
        .mount("/", rocket::routes![api::user::get_users])
        .mount("/", rocket::routes![api::user::add_user])
}
