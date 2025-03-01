use crate::models::course::Course;
use crate::models::deadline::Events;
use crate::models::grade::{GradesOverview, UserGrades};
use crate::models::user::User;
use async_trait::async_trait;

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
