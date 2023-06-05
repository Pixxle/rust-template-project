use crate::models::{post::Post, user::User};
use crate::repository::Database;
use std::sync::Mutex;

pub struct InMemDatabase {
    users: Mutex<Vec<User>>,
    posts: Mutex<Vec<Post>>,
}

pub fn new() -> InMemDatabase {
    InMemDatabase {
        users: Mutex::new(Vec::new()),
        posts: Mutex::new(Vec::new()),
    }
}

impl Database for InMemDatabase {
    fn add_user(&self, user: User) -> User {
        let mut guard = self.users.lock().unwrap();
        let users = &mut *guard;
        let mut resp = user.clone();
        resp.id = Some(users.len() as i32);
        users.push(resp.clone());
        resp
    }
    fn add_post(&self, post: Post) -> Post {
        let mut guard = self.posts.lock().unwrap();
        let posts = &mut *guard;
        let mut resp = post.clone();
        resp.id = Some(posts.len() as i32);
        posts.push(resp.clone());
        resp
    }

    fn get_users(&self) -> Vec<User> {
        let guard = self.users.lock().unwrap();
        let users = &*guard;
        users.clone()
    }

    fn get_posts(&self) -> Vec<Post> {
        let guard = self.posts.lock().unwrap();
        let posts = &*guard;
        posts.clone()
    }

    fn get_user(&self, id: i32) -> Option<User> {
        let guard = self.users.lock().unwrap();
        let users = &*guard;
        users.iter().find(|u| u.id == Some(id)).cloned()
    }

    fn get_post(&self, id: i32) -> Option<Post> {
        let guard = self.posts.lock().unwrap();
        let posts = &*guard;
        posts.iter().find(|p| p.id == Some(id)).cloned()
    }
}
