use async_trait::async_trait;

use crate::domain::entities::user::User;

#[async_trait]
pub trait UserRepository {
    async fn find_user_by_token(&self, token: &String) -> Result<Option<User>, Box<dyn std::error::Error>>;
    async fn update_user_by_token(&self, user: &User, token: &String) -> Result<(), Box<dyn std::error::Error>>;
}