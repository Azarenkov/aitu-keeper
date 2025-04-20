use std::sync::Arc;

use crate::domain::{
    data_providers::data_provider_abstract::DataProviderAbstract,
    entities::{errors::ServiceError, user::User},
    repositories::data_repository_abstract::UserRepositoryAbstract,
};

#[derive(Debug)]
pub struct UserService<T, U>
where
    T: DataProviderAbstract,
    U: UserRepositoryAbstract,
{
    data_provider: Arc<T>,
    pub user_repository: Arc<U>,
}

impl<T, U> UserService<T, U>
where
    T: DataProviderAbstract,
    U: UserRepositoryAbstract,
{
    pub fn new(data_provider: Arc<T>, user_repository: Arc<U>) -> Self {
        Self {
            data_provider,
            user_repository,
        }
    }

    pub async fn update_user(&self, token: &str) -> Result<User, ServiceError> {
        let user = self.data_provider.get_user(token).await?;
        self.user_repository.save_user(&user, token).await?;
        Ok(user)
    }

    pub async fn get_user(&self, token: &str) -> Result<User, ServiceError> {
        let user = self.user_repository.find_user_by_token(token).await?;
        Ok(user)
    }
}
