use crate::models::course::Course;
use crate::models::deadline::Deadline;
use async_trait::async_trait;
use std::error::Error;

#[async_trait(?Send)]
pub trait DeadlineServiceInterface: Send + Sync {
    async fn get_deadlines(&self, token: &str) -> Result<Vec<Deadline>, Box<dyn Error>>;
    async fn update_deadlines(&self, token: &str, courses: &[Course]) -> Result<(), Box<dyn Error>>;
}