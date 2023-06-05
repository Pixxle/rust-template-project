pub mod database;
use crate::models::{post::Post, user::User};

pub trait Database: Send + Sync {
    fn add_user(&self, user: User) -> User;
    fn add_post(&self, post: Post) -> Post;
    fn get_users(&self) -> Vec<User>;
    fn get_posts(&self) -> Vec<Post>;
    fn get_user(&self, id: i32) -> Option<User>;
    fn get_post(&self, id: i32) -> Option<Post>;
}
