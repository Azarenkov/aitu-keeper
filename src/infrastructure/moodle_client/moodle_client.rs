use async_trait::async_trait;
use reqwest::{Client, Error};
use crate::models::course::Course;
use crate::models::user::User;
use crate::services::interfaces::provider_interface::ProviderInterface;

pub struct MoodleClient {
    client: Client,
    base_url: String,
}

impl MoodleClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
}

#[async_trait]
impl ProviderInterface for MoodleClient {
    async fn get_user(&self, token: &str) -> Result<User, reqwest::Error> {
        let url = format!("{}wstoken={}&wsfunction=core_webservice_get_site_info&moodlewsrestformat=json", self.base_url, token);
        let response = self.client.get(&url).send().await?;
        response.json::<User>().await
    }

    async fn valid_token(&self, token: &str) -> Result<(), Error> {
        let url = format!("{}wstoken={}&wsfunction=core_webservice_get_site_info&moodlewsrestformat=json", self.base_url, token);
        let response = self.client.get(&url).send().await?;
        response.json::<User>().await?;
        Ok(())
    }

    async fn get_courses(&self, token: &str, user_id: i64) -> Result<Vec<Course>, Error> {
        let url = format!("{}wstoken={}&wsfunction=core_enrol_get_users_courses&moodlewsrestformat=json&userid={}",
                          self.base_url,
                          token,
                          user_id
        );
        let response = self.client.get(&url).send().await?;
        response.json::<Vec<Course>>().await
    }
}

