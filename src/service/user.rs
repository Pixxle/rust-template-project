use std::sync::Arc;
use crate::repository::database::database_trait::Database;

#[derive(Clone)]
pub struct UserService{
    database: Arc<dyn Database>
}

pub fn new(database: Arc<dyn Database>) -> UserService {
    UserService {
        database: database,
    }
}

impl UserService {
    pub fn add_user(&self, user: &String) {
        self.database.add_user(user);
    }

    pub fn get_users(&self) -> Vec<String> {
        self.database.get_users()
    }
}