use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use crate::models::user::User;
use crate::repositories::interfaces::user_repository_interface::UserRepositoryInterface;
use crate::services::interfaces::user_provider_interface::UserProviderInteface;
use crate::services::interfaces::user_service_interface::UserServiceInterface;

pub struct UserService  {
    user_repository: Arc<dyn UserRepositoryInterface>,
    user_provider: Arc<dyn UserProviderInteface>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepositoryInterface>, http_client: Arc<dyn UserProviderInteface>) -> Self {
        Self { user_repository, user_provider: http_client }
    }
}

#[async_trait(?Send)]
impl UserServiceInterface for UserService {
    async fn create_user(&self, token: &String) -> Result<User, Box<dyn Error>> {
        let is_exist = self.user_repository.is_exist(token).await?;
        if is_exist {
            return Err("User already exist".into());
        }
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
    

    async fn delete_user(&self, token: &String) -> Result<(), Box<dyn Error>> {
        self.user_repository.delete(token).await
    }
}