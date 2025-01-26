use std::error::Error;
use async_trait::async_trait;
use crate::models::token::Token;

#[async_trait(?Send)]
pub trait TokenServiceInterface {
    async fn create_token(&self, token: &Token) -> Result<(), Box<dyn Error>>;
    async fn delete_all(&self, token: &str) -> Result<(), Box<dyn Error>>;
}