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

#[test]
fn test_add_get_post() {
    use crate::repository::database::in_mem_database;
    let database: Arc<dyn Database> = Arc::new(in_mem_database::new());
    let service = new(database.clone());
    service.add_post(&"test".to_string());
    assert_eq!(database.get_posts(), vec!["test".to_string()]);
}
