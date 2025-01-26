use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use futures_util::future::err;
use crate::models::token::Token;
use crate::repositories::interfaces::token_repository_interface::TokenRepositoryInterface;
use crate::repositories::interfaces::user_repository_interface::UserRepositoryInterface;
use crate::services::interfaces::provider_interface::ProviderInterface;
use crate::services::interfaces::token_service_interface::TokenServiceInterface;

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
        let is_valid_token = self.token_provider.valid_token(&token.token).await?;
        match self.token_repository.save(token).await {
            Ok(_) => Ok(()),
            Err(_user_already_exist) => Ok(()),
        }
    }
}
