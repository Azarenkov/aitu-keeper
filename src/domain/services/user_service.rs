use std::sync::Arc;

use crate::{adapters::spi::http::external_user_service::ExternalUserService, domain::{entities::user::User, repositories::user_repository::UserRepository}};

#[derive(Clone)]
pub struct UserService<T: UserRepository> {
    user_repo: Arc<T>,
    external_user_service: Arc<ExternalUserService>,
}

impl <T: UserRepository> UserService<T> {
    pub fn new(user_repo: Arc<T>, external_user_service: Arc<ExternalUserService>) -> Self {
        UserService { user_repo, external_user_service }
    }

    pub async fn update_user(&self, user: &User, token: &String) -> Result<(), Box<dyn std::error::Error>> {
        self.user_repo.update_user_by_token(user, token).await?;
        Ok(())
    }

    pub async fn get_user(&self, token: &String) -> Result<User, Box<dyn std::error::Error>> {
        match self.user_repo.find_user_by_token(token).await? {
            Some(user) => Ok(user),
            None => Err("User not found".into())
        }
    }

    pub async fn fetch_and_save_user(&self, token: &String) -> Result<(), Box<dyn std::error::Error>> {
        let user = self.external_user_service.fetch_external_user(token).await?;
        self.user_repo.update_user_by_token(&user, token).await?;
        Ok(())
    }
}