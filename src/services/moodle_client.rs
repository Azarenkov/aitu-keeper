use async_trait::async_trait;
use reqwest::Client;
use crate::models::user::User;
use crate::services::interfaces::user_provider_interface::UserProvider;

pub struct MoodleClient {
    client: Client,
}

impl MoodleClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl UserProvider for MoodleClient {
    async fn get_user(&self, token: &str) -> Result<User, reqwest::Error> {
        let url = format!("https://moodle.astanait.edu.kz/webservice/rest/server.php?wstoken={}&wsfunction=core_webservice_get_site_info&moodlewsrestformat=json", token);
        let response = self.client.get(&url).send().await?;
        response.json::<User>().await
    }
}
