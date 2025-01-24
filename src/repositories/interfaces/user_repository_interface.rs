use std::error::Error;
use crate::models::user::User;

pub trait UserRepositoryInterface {
    async fn find_by_token(&self, token: &str) -> Result<User, dyn Error>;
    async fn find_all(&self) -> Result<Vec<User>, dyn Error>;
    async fn create(&self, user: &User, token: &String) -> Result<User, dyn Error>;
    async fn update(&self, user: &User, token: &String) -> Result<User, dyn Error>;
    async fn delete(&self, token: &String) -> Result<(), dyn Error>;
}