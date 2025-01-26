use async_trait::async_trait;
use crate::models::course::Course;

#[async_trait]
pub trait CourseProviderInteface: Send + Sync  {
    async fn get_courses(&self, token: &str, user_id: i64) -> Result<Vec<Course>, reqwest::Error>;
}