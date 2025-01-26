use std::error::Error;
use async_trait::async_trait;
use crate::models::token::Token;

#[async_trait]
pub trait TokenRepositoryInterface: Send + Sync {
    async fn save(&self, token: &Token) -> Result<(), Box<dyn Error>>;
}