use std::sync::Mutex;
use super::database_trait::Database;

pub struct InMemDatabase {
    users: Mutex<Vec<String>>,
    posts: Mutex<Vec<String>>,
}


pub fn new() -> InMemDatabase {
    InMemDatabase {
        users: Mutex::new(Vec::new()),
        posts: Mutex::new(Vec::new()),
    }
}

impl Database for InMemDatabase {
     fn add_user(&self, u: &String) {
        let mut guard = self.users.lock().unwrap();
        let users = &mut *guard;
        users.push(u.clone());
    }
     fn add_post(&self, post: &String) {
        let mut guard = self.posts.lock().unwrap();
        let posts = &mut *guard;
        posts.push(post.clone());
    }

     fn get_users(&self) -> Vec<String> {
        let guard = self.users.lock().unwrap();
        let users = &*guard;
        users.clone()
    }

     fn get_posts(&self) -> Vec<String> {
        let guard = self.posts.lock().unwrap();
        let posts = &*guard;
        posts.clone()
    } 
}