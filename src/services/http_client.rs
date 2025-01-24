use reqwest::Client;
use crate::models::user::User;
use crate::services::interfaces::http_client_interface::HttpClientInteface;

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl HttpClientInteface for HttpClient {
    async fn get_user(&self, token: &str) -> Result<User, reqwest::Error> {
        let url = format!("https://moodle.astanait.edu.kz/webservice/rest/server.php?wstoken={}&wsfunction=core_webservice_get_site_info&moodlewsrestformat=json", token);
        let response = self.client.get(&url).send().await?;
        response.json::<User>().await
    }
}
