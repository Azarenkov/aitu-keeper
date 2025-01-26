use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use crate::models::user::User;
use crate::repositories::interfaces::user_repository_interface::UserRepositoryInterface;
use crate::services::interfaces::provider_interface::ProviderInterface;
use crate::services::interfaces::user_service_interface::UserServiceInterface;

pub struct UserService  {
    user_repository: Arc<dyn UserRepositoryInterface>,
    user_provider: Arc<dyn ProviderInterface>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepositoryInterface>, http_client: Arc<dyn ProviderInterface>) -> Self {
        Self { user_repository, user_provider: http_client }
    }
}

#[async_trait(?Send)]
impl UserServiceInterface for UserService {
    async fn create_user(&self, token: &String) -> Result<User, Box<dyn Error>> {
        match self.user_provider.get_user(token).await {
            Ok(user) => {
                self.user_repository.save(&user, token).await?;
                Ok(user)
            },
            Err(_) => return Err("Invalid token".into()),
        }
        
    }

    async fn find_user_by_token(&self, token: &str) -> Result<User, Box<dyn Error>> {
        self.user_repository.find_by_token(token).await
    }
    
}