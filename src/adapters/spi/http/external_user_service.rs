use crate::domain::entities::user::User;
use reqwest::Client;
use std::error::Error;

pub struct ExternalUserService {
    client: Client,
}

impl ExternalUserService {
    pub fn new(client: Client) -> Self {
        ExternalUserService { client }
    }

    pub async fn fetch_external_user(&self, token: &String) -> Result<User, Box<dyn Error>> {
        let url = format!("https://moodle.astanait.edu.kz/webservice/rest/server.php?wstoken={}&wsfunction=core_webservice_get_site_info&moodlewsrestformat=json", token);
        let response = self.client.get(&url).send().await?.json::<User>().await?;
        println!("{:?}", response);
        Ok(response)
    }
}
