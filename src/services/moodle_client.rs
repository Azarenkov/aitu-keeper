use async_trait::async_trait;
use reqwest::Client;
use crate::models::user::User;
use crate::services::interfaces::user_provider_interface::UserProvider;

pub struct MoodleClient<'a> {
    client: Client,
    base_url: &'a str,
}

impl<'a> MoodleClient<'a> {
    pub fn new(base_url: &'a str) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
}

#[async_trait]
impl<'a> UserProvider for MoodleClient<'a> {
    async fn get_user(&self, token: &str) -> Result<User, reqwest::Error> {
        let url = format!("{}wstoken={}&wsfunction=core_webservice_get_site_info&moodlewsrestformat=json", self.base_url, token);
        let response = self.client.get(&url).send().await?;
        response.json::<User>().await
    }
}
