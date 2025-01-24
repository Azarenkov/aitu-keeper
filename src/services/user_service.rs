use std::error::Error;
use std::sync::Arc;
use crate::models::user::User;
use crate::repositories::interfaces::user_repository_interface::UserRepositoryInterface;
use crate::services::interfaces::http_client_interface::HttpClientInteface;

pub struct UserService<R: UserRepositoryInterface, T: HttpClientInteface> {
    user_repository: Arc<R>,
    http_client: Arc<T>,
}

impl<R: UserRepositoryInterface, T: HttpClientInteface> UserService<R, T> {
    pub fn new(user_repository: Arc<R>, http_client: Arc<T>) -> Self {
        UserService { user_repository, http_client }
    }

    pub async fn create_user(&self, token: &String) -> Result<(), Box<dyn Error>> {
        let is_exist = self.user_repository.is_exist(token).await?;
        if is_exist {
            return Ok(());
        }
        match self.http_client.get_user(token).await {
            Ok(user) => self.user_repository.create(&user, token).await?,
            Err(_) => return Err("Invalid token".into()),
        }
        Ok(())
    }

    pub async fn find_user_by_token(&self, token: &str) -> Result<User, Box<dyn Error>> {
        self.user_repository.find_by_token(token).await
    }

    pub async fn update_user(&self, user: &User, token: &String) -> Result<(), Box<dyn Error>> {
        self.user_repository.update(user, token).await?;
        Ok(())
    }

    pub async fn delete_user(&self, token: &String) -> Result<(), Box<dyn Error>> {
        self.user_repository.delete(token).await
    }
}