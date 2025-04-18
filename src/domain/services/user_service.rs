use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;

use crate::domain::{
    data_providers::data_provider_abstract::DataProviderAbstract,
    entities::{errors::ServiceError, user::User},
    repositories::data_repository_abstract::UserRepositoryAbstract,
};

#[async_trait]
pub trait UserServiceAbstract: Send + Sync + Debug {
    async fn update_user(&self, token: &str) -> Result<User, ServiceError>;
    async fn get_user(&self, token: &str) -> Result<User, ServiceError>;
}

#[derive(Debug)]
pub struct UserService {
    data_provider: Arc<dyn DataProviderAbstract>,
    pub user_repository: Arc<dyn UserRepositoryAbstract>,
}

impl UserService {
    pub fn new(
        data_provider: Arc<dyn DataProviderAbstract>,
        user_repository: Arc<dyn UserRepositoryAbstract>,
    ) -> Self {
        Self {
            data_provider,
            user_repository,
        }
    }
}

#[async_trait]
impl UserServiceAbstract for UserService {
    async fn update_user(&self, token: &str) -> Result<User, ServiceError> {
        let user = self.data_provider.get_user(token).await?;
        self.user_repository.save_user(&user, token).await?;
        Ok(user)
    }

    async fn get_user(&self, token: &str) -> Result<User, ServiceError> {
        let user = self.user_repository.find_user_by_token(token).await?;
        Ok(user)
    }
}
