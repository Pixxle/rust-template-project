pub trait Database: Send + Sync {
    fn add_user(&self, u: &String);
    fn add_post(&self, post: &String);
    fn get_users(&self) -> Vec<String>;
    fn get_posts(&self) -> Vec<String>;
}