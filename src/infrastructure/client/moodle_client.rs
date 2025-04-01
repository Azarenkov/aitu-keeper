use std::time::Duration;

use crate::models::course::Course;
use crate::models::deadline::Events;
use crate::models::grade::{GradesOverview, UserGrades};
use crate::models::user::User;
use crate::services::provider_interfaces::DataProviderInterface;
use async_trait::async_trait;
use reqwest::{Client, Error};

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
}

#[async_trait]
impl DataProviderInterface for MoodleClient {
    async fn get_user(&self, token: &str) -> Result<User, Error> {
        let url = format!(
            "{}wstoken={}&wsfunction=core_webservice_get_site_info{}",
            self.base_url, token, self.format
        );
        let response = self.client.get(&url).send().await?;
        response.json::<User>().await
    }

    async fn valid_token(&self, token: &str) -> Result<(), Error> {
        let url = format!(
            "{}wstoken={}&wsfunction=core_webservice_get_site_info{}",
            self.base_url, token, self.format
        );
        let response = self.client.get(&url).send().await?;
        response.json::<User>().await?;
        Ok(())
    }

    async fn get_courses(&self, token: &str, user_id: i64) -> Result<Vec<Course>, Error> {
        let url = format!(
            "{}wstoken={}&wsfunction=core_enrol_get_users_courses{}&userid={}",
            self.base_url, token, self.format, user_id,
        );
        let response = self.client.get(&url).send().await?;
        response.json::<Vec<Course>>().await
    }

    async fn get_grades_by_course_id(
        &self,
        token: &str,
        user_id: i64,
        course_id: i64,
    ) -> Result<UserGrades, Error> {
        let url = format!(
            "{}wstoken={}&wsfunction=gradereport_user_get_grade_items{}&userid={}&courseid={}",
            self.base_url, token, self.format, user_id, course_id
        );
        let response = self.client.get(&url).send().await?;
        response.json::<UserGrades>().await
    }

    async fn get_deadline_by_course_id(
        &self,
        token: &str,
        course_id: i64,
    ) -> Result<Events, Error> {
        let url = format!(
            "{}wstoken={}&wsfunction=core_calendar_get_action_events_by_course{}&courseid={}",
            self.base_url, token, self.format, course_id,
        );
        let response = self.client.get(&url).send().await?;
        response.json::<Events>().await
    }

    async fn get_grades_overview(&self, token: &str) -> Result<GradesOverview, Error> {
        let url = format!(
            "{}wstoken={}&wsfunction=gradereport_overview_get_course_grades{}",
            self.base_url, token, self.format
        );
        let response = self.client.get(&url).send().await?;
        println!("{:?}", response);
        response.json::<GradesOverview>().await
    }
}
