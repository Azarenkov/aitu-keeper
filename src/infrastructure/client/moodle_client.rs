use std::time::Duration;

use crate::models::course::Course;
use crate::models::deadline::Events;
use crate::models::grade::{GradesOverview, UserGrades};
use crate::models::user::User;
use crate::services::provider_interfaces::DataProviderInterface;
use async_trait::async_trait;
use reqwest::Client;

use super::errors::ResponseError;

pub struct MoodleClient {
    client: Client,
    base_url: String,
    format: String,
}

impl MoodleClient {
    pub fn new(base_url: String, format: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(15))
                .build()
                .unwrap(),
            base_url,
            format,
        }
    }

    async fn send_request<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
        token: &str,
    ) -> Result<T, ResponseError> {
        let mut attempt = 0;
        loop {
            let response = self.client.get(url).send().await;
            match response {
                Ok(value) => match value.json::<T>().await {
                    Ok(value) => return Ok(value),
                    Err(_) => return Err(ResponseError::InvalidToken(token.to_string())),
                },
                Err(e) => {
                    if attempt >= 2 {
                        return Err(e.into());
                    }
                }
            }
            attempt += 1;
            tokio::time::sleep(Duration::from_secs(2)).await;
            continue;
        }
    }
}

#[async_trait]
impl DataProviderInterface for MoodleClient {
    async fn get_user(&self, token: &str) -> Result<User, ResponseError> {
        let url = format!(
            "{}wstoken={}&wsfunction=core_webservice_get_site_info{}",
            self.base_url, token, self.format
        );
        self.send_request(&url, token).await
    }

    async fn valid_token(&self, token: &str) -> Result<(), ResponseError> {
        let url = format!(
            "{}wstoken={}&wsfunction=core_webservice_get_site_info{}",
            self.base_url, token, self.format
        );
        self.send_request::<User>(&url, token).await?;
        Ok(())
    }

    async fn get_courses(&self, token: &str, user_id: i64) -> Result<Vec<Course>, ResponseError> {
        let url = format!(
            "{}wstoken={}&wsfunction=core_enrol_get_users_courses{}&userid={}",
            self.base_url, token, self.format, user_id,
        );
        self.send_request(&url, token).await
    }

    async fn get_grades_by_course_id(
        &self,
        token: &str,
        user_id: i64,
        course_id: i64,
    ) -> Result<UserGrades, ResponseError> {
        let url = format!(
            "{}wstoken={}&wsfunction=gradereport_user_get_grade_items{}&userid={}&courseid={}",
            self.base_url, token, self.format, user_id, course_id
        );
        self.send_request(&url, token).await
    }

    async fn get_deadline_by_course_id(
        &self,
        token: &str,
        course_id: i64,
    ) -> Result<Events, ResponseError> {
        let url = format!(
            "{}wstoken={}&wsfunction=core_calendar_get_action_events_by_course{}&courseid={}",
            self.base_url, token, self.format, course_id,
        );
        self.send_request(&url, token).await
    }

    async fn get_grades_overview(&self, token: &str) -> Result<GradesOverview, ResponseError> {
        let url = format!(
            "{}wstoken={}&wsfunction=gradereport_overview_get_course_grades{}",
            self.base_url, token, self.format
        );
        self.send_request(&url, token).await
    }
}
