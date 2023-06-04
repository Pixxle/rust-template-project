use std::sync::Arc;
use rocket::{self};

mod config;
mod api;
mod service;
mod repository;


#[rocket::launch]
fn rocket() -> _ { 
        let config = config::parse_config();

        let db: Arc<dyn repository::database::database_trait::Database> = match config.environment {
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
