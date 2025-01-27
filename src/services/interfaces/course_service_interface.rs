use std::error::Error;
use async_trait::async_trait;
use crate::models::course::Course;
use crate::models::user::User;

#[async_trait(?Send)]
pub trait CourseServiceInteface: Send + Sync  {
    async fn get_courses(&self, token: &str) -> Result<Vec<Course>, Box<dyn Error>>;
    async fn update_courses(&self, token: &str, user: &User) -> Result<Vec<Course>, Box<dyn Error>>;
}