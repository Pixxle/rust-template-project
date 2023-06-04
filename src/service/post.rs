use crate::repository::database::database_trait::Database;
use std::sync::Arc;

#[derive(Clone)]
pub struct PostService {
    database: Arc<dyn Database>,
}

pub fn new(database: Arc<dyn Database>) -> PostService {
    PostService { database: database }
}

impl PostService {
    pub fn add_post(&self, post: &String) {
        self.database.add_post(post);
    }

    pub fn get_posts(&self) -> Vec<String> {
        self.database.get_posts()
    }
}
