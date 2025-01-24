use std::error::Error;
use std::sync::Arc;
use crate::models::user::User;
use crate::repositories::interfaces::user_repository_interface::UserRepositoryInterface;

pub struct UserService<R: UserRepositoryInterface> {
    user_repository: Arc<R>,
}

impl<R: UserRepositoryInterface> UserService<R> {
    pub fn new(user_repository: Arc<R>) -> Self {
        UserService { user_repository }
    }

    pub async fn create_user(&self, token: &String) -> Result<User, Box<dyn Error>> {
        self.user_repository.create(user, token).await
    }

    pub async fn find_user_by_token(&self, token: &str) -> Result<User, Box<dyn Error>> {
        self.user_repository.find_by_token(token).await
    }

    pub async fn update_user(&self, user: &User, token: &String) -> Result<User, Box<dyn Error>> {
        self.user_repository.update(user, token).await
    }

    pub async fn delete_user(&self, token: &String) -> Result<(), Box<dyn Error>> {
        self.user_repository.delete(token).await
    }
}