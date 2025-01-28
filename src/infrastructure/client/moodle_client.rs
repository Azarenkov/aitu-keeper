use crate::models::course::course_model::Course;
use crate::models::grade::grade_model::{GradesOverview, UserGrades};
use crate::models::user::user_model::User;
use async_trait::async_trait;
use reqwest::{Client, Error};
use crate::models::deadline::deadline_model::Events;
use crate::services::interfaces::ProviderInterface;

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

    async fn get_grades_by_course_id(&self, token: &str, user_id: i64, course_id: i64) -> Result<UserGrades, Error> {
        let url = format!("{}wstoken={}&wsfunction=gradereport_user_get_grade_items&moodlewsrestformat=json&userid={}&courseid={}",
            self.base_url,
            token,
            user_id,
            course_id
        );
        let response = self.client.get(&url).send().await?;
        response.json::<UserGrades>().await
    }

    async fn get_deadline_by_course_id(&self, token: &str, course_id: i64) -> Result<Events, Error> {
        let url = format!("{}wstoken={}&wsfunction=core_calendar_get_action_events_by_course&moodlewsrestformat=json&courseid={}",
            self.base_url,
            token,
            course_id,
        );
        let response = self.client.get(&url).send().await?;
        response.json::<Events>().await
    }

    async fn get_grades_overview(&self, token: &str) -> Result<GradesOverview, Error> {
        let url = format!("{}wstoken={}&wsfunction=gradereport_overview_get_course_grades&moodlewsrestformat=json",
            self.base_url,
            token
        );
        let response = self.client.get(&url).send().await?;
        response.json::<GradesOverview>().await
    }
}

