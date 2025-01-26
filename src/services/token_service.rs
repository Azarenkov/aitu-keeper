use crate::models::token::Token;
use crate::repositories::interfaces::token_repository_interface::TokenRepositoryInterface;
use crate::services::interfaces::provider_interface::ProviderInterface;
use crate::services::interfaces::token_service_interface::TokenServiceInterface;
use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;

pub struct TokenService {
    token_repository: Arc<dyn TokenRepositoryInterface>,
    token_provider: Arc<dyn ProviderInterface>,
}

impl TokenService {
    pub fn new(token_repository: Arc<dyn TokenRepositoryInterface>, token_provider: Arc<dyn ProviderInterface>) -> Self {
        Self { token_repository, token_provider }
    }
}

#[async_trait(?Send)]
impl TokenServiceInterface for TokenService {
    async fn create_token(&self, token: &Token) -> Result<(), Box<dyn Error>> {
        self.token_provider.valid_token(&token.token).await?;
        match self.token_repository.save(token).await {
            Ok(_) => Ok(()),
            Err(_) => Err("User already exist".into()),
        }
    }

    async fn delete_all(&self, token: &str) -> Result<(), Box<dyn Error>> {
        self.token_repository.delete(token).await
    }
}
