use std::error::Error;
use async_trait::async_trait;
use crate::models::course::course_model::Course;
use crate::models::user::user_model::User;

#[async_trait]
pub trait CourseServiceInteface: Send + Sync  {
    async fn get_courses(&self, token: &str) -> Result<Vec<Course>, Box<dyn Error>>;
    async fn update_courses(&self, token: &str, user: &User) -> Result<Vec<Course>, Box<dyn Error>>;
}