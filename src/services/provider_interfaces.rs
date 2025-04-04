use crate::models::course::Course;
use crate::models::deadline::Events;
use crate::models::grade::{GradesOverview, UserGrades};
use crate::models::user::User;
use anyhow::Result;
use async_trait::async_trait;
use core::fmt::Debug;
use fcm_rs::models::Message;

#[async_trait]
pub trait DataProviderInterface: Send + Sync {
    async fn get_user(&self, token: &str) -> Result<User, reqwest::Error>;
    async fn valid_token(&self, token: &str) -> Result<(), reqwest::Error>;
    async fn get_courses(&self, token: &str, user_id: i64) -> Result<Vec<Course>, reqwest::Error>;
    async fn get_grades_by_course_id(
        &self,
        token: &str,
        user_id: i64,
        course_id: i64,
    ) -> Result<UserGrades, reqwest::Error>;
    async fn get_deadline_by_course_id(
        &self,
        token: &str,
        course_id: i64,
    ) -> Result<Events, reqwest::Error>;
    async fn get_grades_overview(&self, token: &str) -> Result<GradesOverview, reqwest::Error>;
}

impl Debug for dyn DataProviderInterface {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "DataProviderInterface{{}}")
    }
}

#[async_trait]
pub trait NotificationProviderInterface: Send + Sync {
    async fn send_notification(&self, message: Message) -> Result<()>;
    fn create_message(&self, device_token: &str, title: &str, body: &str) -> Message;
}

impl Debug for dyn NotificationProviderInterface {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "NotificationProviderInterface{{}}")
    }
}
