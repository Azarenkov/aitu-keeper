use std::error::Error;
use async_trait::async_trait;
use crate::models::user::User;

#[async_trait(?Send)]
pub trait UserServiceInterface {
    async fn create_user(&self, token: &String) -> Result<User, Box<dyn Error>>;
    async fn get_user(&self, token: &str) -> Result<User, Box<dyn Error>>;
}