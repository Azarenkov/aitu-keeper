use std::error::Error;
use async_trait::async_trait;
use crate::models::user::User;

#[async_trait(?Send)]
pub trait UserServiceInterface {
    async fn create_user(&self, token: &String) -> Result<(), Box<dyn Error>>;
    async fn find_user_by_token(&self, token: &str) -> Result<User, Box<dyn Error>>;
    async fn update_user(&self, user: &User, token: &String) -> Result<(), Box<dyn Error>>;
    async fn delete_user(&self, token: &String) -> Result<(), Box<dyn Error>>;
}