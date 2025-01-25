use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use crate::models::user::User;
use crate::repositories::interfaces::user_repository_interface::UserRepositoryInterface;
use crate::services::interfaces::user_provider_interface::UserProvider;
use crate::services::interfaces::user_service_interface::UserServiceInterface;

pub struct UserService  {
    user_repository: Arc<dyn UserRepositoryInterface>,
    user_provider: Arc<dyn UserProvider>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepositoryInterface>, http_client: Arc<dyn UserProvider>) -> Self {
        UserService { user_repository, user_provider: http_client }
    }
}

#[async_trait(?Send)]
impl UserServiceInterface for UserService {
    async fn create_user(&self, token: &String) -> Result<(), Box<dyn Error>> {
        let is_exist = self.user_repository.is_exist(token).await?;
        if is_exist {
            return Ok(());
        }
        match self.user_provider.get_user(token).await {
            Ok(user) => self.user_repository.create(&user, token).await?,
            Err(_) => return Err("Invalid token".into()),
        }
        Ok(())
    }

    async fn find_user_by_token(&self, token: &str) -> Result<User, Box<dyn Error>> {
        self.user_repository.find_by_token(token).await
    }

    async fn update_user(&self, user: &User, token: &String) -> Result<(), Box<dyn Error>> {
        self.user_repository.update(user, token).await?;
        Ok(())
    }

    async fn delete_user(&self, token: &String) -> Result<(), Box<dyn Error>> {
        self.user_repository.delete(token).await
    }
}