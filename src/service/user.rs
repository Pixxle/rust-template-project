use crate::repository::database::database_trait::Database;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserService {
    database: Arc<dyn Database>,
}

pub fn new(database: Arc<dyn Database>) -> UserService {
    UserService { database: database }
}

impl UserService {
    pub fn add_user(&self, user: &String) {
        self.database.add_user(user);
    }

    pub fn get_users(&self) -> Vec<String> {
        self.database.get_users()
    }
}

#[test]
fn test_add_get_user() {
    use crate::repository::database::in_mem_database;
    let database: Arc<dyn Database> = Arc::new(in_mem_database::new());
    let service = new(database.clone());
    service.add_user(&"test".to_string());
    assert_eq!(database.get_users(), vec!["test".to_string()]);
}
