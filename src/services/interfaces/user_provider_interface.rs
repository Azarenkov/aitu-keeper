use async_trait::async_trait;
use crate::models::user::User;

#[async_trait]
pub trait UserProviderInteface: Send + Sync  {
    async fn get_user(&self, token: &str) -> Result<User, reqwest::Error>;
}