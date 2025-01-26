use std::error::Error;
use async_trait::async_trait;
use crate::models::course::Course;

#[async_trait]
pub trait CourseRepositoryInterface: Send + Sync {
    async fn save(&self, token: &str, courses: &Vec<Course>) -> Result<(), Box<dyn Error>>;
    async fn find_by_token(&self, token: &str) -> Result<Vec<Course>, Box<dyn Error>>;
}