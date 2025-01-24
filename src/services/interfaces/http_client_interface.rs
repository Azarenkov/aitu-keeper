use crate::models::user::User;

pub trait HttpClientInteface {
    async fn get_user(&self, token: &str) -> Result<User, reqwest::Error>;
}